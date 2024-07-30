#![warn(clippy::all)]
use color_eyre::eyre::Result;
use log::Level;
use std::io::Write;
use url::Url;
use yansi::{Color, Condition, Paint, Style};

pub mod archiver;
pub mod args;
pub mod directory;
pub mod emojis;
pub mod requests;

use crate::archiver::ZipArchiver;
use crate::args::Args;
use crate::directory::Directory;
use clap::Parser;

static STEP: Style = Color::Green.bold();

async fn download_and_zip(url: &str, args: &Args) -> Result<()> {
    let steps = if args.zipped { 3 } else { 2 };

    log::info!(
        "{} {} Validating url...",
        format!("[1/{steps}]").paint(STEP),
        emojis::LOOKING_GLASS
    );

    let url = Url::parse(url)?;
    let data = Directory::new(url, args.path.clone())?;

    log::info!(
        "{} {} Downloading...",
        format!("[2/{steps}]").paint(STEP),
        emojis::TRUCK
    );

    requests::fetch_and_download(&data).await?;

    if args.zipped {
        log::info!(
            "{} {} Zipping...",
            format!("[3/{steps}]").paint(STEP),
            emojis::PACKAGE
        );

        let dest = format!("{}.zip", &data.root);
        let zipper = ZipArchiver::new(&data.root, &dest);
        zipper.run()?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let color = match record.level() {
                Level::Info => return writeln!(buf, "{}", record.args()),

                Level::Error => Color::Red,
                Level::Warn => Color::Yellow,
                Level::Debug => Color::Magenta,
                Level::Trace => Color::Blue,
            };
            writeln!(buf, "[{}] {}", record.level().paint(color), record.args())
        })
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
        "Downloaded {:?} director{}.",
        &url_count,
        if url_count == 1 { "y" } else { "ies" },
    );

    Ok(())
}
