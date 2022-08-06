#![warn(clippy::all)]
use crossterm::style::Stylize;
use std::io::Error;
use std::process;

pub mod arg_parser;
pub mod file_archiver;
pub mod output;
pub mod parser;
pub mod requests;

use crate::arg_parser::create_args_parser;
use crate::file_archiver::ZipArchiver;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = create_args_parser().get_matches();

    let urls = matches.values_of("url").unwrap().collect::<Vec<&str>>();
    for url in &urls {
        println!("Getting: {:?}", url);
        println!(
            "{} {}Validating url...",
            "[1/3]".bold().dim(),
            output::LOOKING_GLASS
        );

        let path = match parser::parse_url(url) {
            Ok(path) => path,
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                process::exit(0);
            }
        };

        let data = match parser::parse_path(&path) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                process::exit(0);
            }
        };

        println!("{} {}Downloading...", "[2/3]".bold().dim(), output::TRUCK);

        match requests::fetch_data(&data).await {
            Err(err) => eprintln!("{}", err.to_string().red()),
            Ok(_) => println!(
                "{} {}Downloaded Successfully.",
                "[3/3]".bold().dim(),
                output::SPARKLES
            ),
        };

        if matches.is_present("zip") {
            let dst_zip = format!("{}.zip", &data.root);
            let zipper = ZipArchiver::new(&data.root, &dst_zip);
            zipper.run().unwrap();
        }
    }

    println!(
        "
        [+] Downloaded {:?} file(s).",
        &urls.len()
    );

    Ok(())
}
