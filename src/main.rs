#![warn(clippy::all)]
use colored::Colorize;
use console::style;
use std::process;

pub mod arg_parser;
pub mod file_archiver;
pub mod output;
pub mod parser;
pub mod requests;

use crate::arg_parser::create_args_parser;
use crate::file_archiver::ZipArchiver;

#[tokio::main]
async fn main() {
    let matches = create_args_parser().get_matches();

    let urls = matches.values_of("url").unwrap().collect::<Vec<&str>>();
    for url in &urls {
        println!(
            "
            Getting: {:?}",
            url
        );
        println!(
            "{} {}Validating url...",
            style("[1/3]").bold().dim(),
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

        println!(
            "{} {}Downloading...",
            style("[2/3]").bold().dim(),
            output::TRUCK
        );

        match requests::fetch_data(&data).await {
            Err(err) => eprintln!("{}", err.to_string().red()),
            Ok(_) => println!(
                "{} {}Downloaded Successfully.",
                style("[3/3]").bold().dim(),
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
}
