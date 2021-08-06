use clap::{load_yaml, App};
use colored::Colorize;
use console::style;
use std::process;

mod output;
mod utils;

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let url = matches.value_of("URL");

    println!(
        "{} {}Validating url...",
        style("[1/2]").bold().dim(),
        output::LOOKING_GLASS
    );
    let path = match utils::parser::parse_url(url.unwrap()) {
        Ok(path) => path,
        Err(err) => {
            println!("{}", err.to_string().red());
            process::exit(0);
        }
    };

    let data = match utils::parser::parse_path(&path) {
        Ok(data) => data,
        Err(err) => {
            println!("{}", err.to_string().red());
            process::exit(0);
        }
    };

    println!(
        "{} {}Downloading...",
        style("[2/2]").bold().dim(),
        output::TRUCK
    );

    match utils::parser::fetch_data(data).await {
        Err(err) => println!("{}", err.to_string().red()),
        Ok(_) => println!(
            "{} {}Cloned Successfully.",
            style("[3/3]").bold().dim(),
            output::SPARKLES
        ),
    };
}
