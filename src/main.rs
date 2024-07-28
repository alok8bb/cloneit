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

    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .format_target(false)
        .format_level(false)
        .filter_level(if args.quiet {
            log::LevelFilter::Warn
        } else {
            log::LevelFilter::Info
        })
        .parse_default_env()
        .init();

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

    let url_count = args.urls.len();
    for (i, url) in args.urls.iter().enumerate() {
        log::info!(
            "{} Cloning {url:?}...",
            format!("[{}/{}]", i + 1, url_count + 1).bold().blue()
        );

        let steps = 3 + (if args.zipped { 2 } else { 0 });

        log::info!(
            "{} {} Validating url...",
            format!("[1/{steps}]").bold().yellow(),
            output::LOOKING_GLASS
        );

        let path = match parser::parse_url(url) {
            Ok(path) => path,
            Err(err) => {
                log::error!("{}", err.to_string().red());
                process::exit(0);
            }
        };

        let data = match parser::parse_path(&path, args.path.clone()) {
            Ok(data) => data,
            Err(err) => {
                log::error!("{}", err.to_string().red());
                process::exit(0);
            }
        };

        log::info!(
            "{} {} Downloading...",
            format!("[2/{steps}]").bold().yellow(),
            output::TRUCK
        );

        match requests::fetch_data(&data).await {
            Err(err) => {
                log::error!("{}", err.to_string().red());
                process::exit(0);
            }
            Ok(_) => log::info!(
                "{} {} Downloaded successfully.",
                format!("[3/{steps}]").bold().yellow(),
                output::SPARKLES
            ),
        };

        if args.zipped {
            log::info!(
                "{} {} Zipping...",
                format!("[4/{steps}]").bold().yellow(),
                output::PACKAGE
            );

            let dst_zip = format!("{}.zip", &data.root);
            let zipper = ZipArchiver::new(&data.root, &dst_zip);
            match zipper.run() {
                Ok(_) => log::info!(
                    "{} {} Zipped successfully.",
                    format!("[5/{steps}]").bold().yellow(),
                    output::SPARKLES
                ),
                Err(ZipError::FileNotFound) => {
                    log::error!("{}", "Failed to zip files".bold().red())
                }
                Err(e) => log::error!("{}", e.to_string().bold().red()),
            }
        }
    }

    log::info!(
        "{} Downloaded {:?} director{}.",
        format!("[{}/{}]", url_count + 1, url_count + 1)
            .bold()
            .blue(),
        &url_count,
        if url_count == 1 { "y" } else { "ies" },
    );

    Ok(())
}
