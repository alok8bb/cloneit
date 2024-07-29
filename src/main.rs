#![warn(clippy::all)]
use color_eyre::eyre::Result;
use directory::Directory;
use url::Url;
use yansi::{Condition, Paint};

pub mod archiver;
pub mod args;
pub mod directory;
pub mod emojis;
pub mod requests;

use crate::archiver::ZipArchiver;
use crate::args::Args;
use clap::Parser;

async fn download_and_zip(url: &str, args: &Args) -> Result<()> {
    let steps = 3 + if args.zipped { 2 } else { 0 };

    log::info!(
        "{} {} Validating url...",
        format!("[1/{steps}]").bold().yellow(),
        emojis::LOOKING_GLASS
    );

    let url = Url::parse(url)?;
    let data = Directory::new(url, args.path.clone())?;

    log::info!(
        "{} {} Downloading...",
        format!("[2/{steps}]").bold().yellow(),
        emojis::TRUCK
    );

    requests::fetch_and_download(&data).await?;

    log::info!(
        "{} {} Downloaded successfully.",
        format!("[3/{steps}]").bold().yellow(),
        emojis::SPARKLES
    );

    if args.zipped {
        log::info!(
            "{} {} Zipping...",
            format!("[4/{steps}]").bold().yellow(),
            emojis::PACKAGE
        );

        let dst_zip = format!("{}.zip", &data.root);
        let zipper = ZipArchiver::new(&data.root, &dst_zip);

        zipper.run()?;

        log::info!(
            "{} {} Zipped successfully.",
            format!("[5/{steps}]").bold().yellow(),
            emojis::SPARKLES
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

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
            format!("[{}/{}]", i + 1, url_count).bold().blue()
        );

        download_and_zip(url, &args).await?;
    }

    log::info!(
        "{} Downloaded {:?} director{}.",
        format!("[{}/{}]", url_count, url_count).bold().blue(),
        &url_count,
        if url_count == 1 { "y" } else { "ies" },
    );

    Ok(())
}
