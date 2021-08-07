use clap::{load_yaml, App};
use colored::Colorize;
use console::style;
use std::process;

pub mod output;
pub mod parser;
pub mod requests;

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let url = matches.value_of("URL");

    println!(
        "{} {}Validating url...",
        style("[1/3]").bold().dim(),
        output::LOOKING_GLASS
    );
    let path = match parser::parse_url(url.unwrap()) {
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

    match requests::fetch_data(data).await {
        Err(err) => eprintln!("{}", err.to_string().red()),
        Ok(_) => println!(
            "{} {}Downloaded Successfully.",
            style("[3/3]").bold().dim(),
            output::SPARKLES
        ),
    };
}
