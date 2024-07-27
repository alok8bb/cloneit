#![warn(clippy::all)]
use std::io::Error;
use std::process;
use yansi::{Condition, Paint};
use zip::result::ZipError;

pub mod args;
pub mod file_archiver;
pub mod output;
pub mod parser;
pub mod requests;

use crate::args::CommandArgs;
use crate::file_archiver::ZipArchiver;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = CommandArgs::parse();

    const USE_COLOR: Condition = Condition(|| {
        if std::env::var_os("FORCE_COLOR").is_some() {
            true
        } else if std::env::var_os("NO_COLOR").is_some() {
            false
        } else {
            true
        }
    });

    yansi::whenever(Condition::cached((USE_COLOR)()));

    for url in &args.urls {
        println!("Getting: {:?}", url);
        println!(
            "{} {}Validating url...",
            "[1/3]".bold().yellow(),
            output::LOOKING_GLASS
        );

        let path = match parser::parse_url(url) {
            Ok(path) => path,
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                process::exit(0);
            }
        };

        let data = match parser::parse_path(&path, args.path.clone()) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                process::exit(0);
            }
        };

        println!(
            "{} {}Downloading...",
            "[2/3]".bold().yellow(),
            output::TRUCK
        );

        match requests::fetch_data(&data).await {
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                process::exit(0);
            }
            Ok(_) => println!(
                "{} {}Downloaded Successfully.",
                "[3/3]".bold().yellow(),
                output::SPARKLES
            ),
        };

        if args.zipped {
            let dst_zip = format!("{}.zip", &data.root);
            let zipper = ZipArchiver::new(&data.root, &dst_zip);
            match zipper.run() {
                Ok(_) => (),
                Err(ZipError::FileNotFound) => {
                    eprintln!("{}", "\ncould not zip the downloaded file".bold().red())
                }
                Err(e) => eprintln!("{}", e.to_string().bold().red()),
            }
        }
    }

    println!(
        "
        [+] Downloaded {:?} dir(s).",
        &args.urls.len()
    );

    Ok(())
}
