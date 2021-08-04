use clap::{load_yaml, App};
use std::process;

mod lib;

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let url = matches.value_of("URL");

    let path = match lib::parse_url(url.unwrap()) {
        Ok(path) => path,
        Err(err) => {
            println!("{}", err.to_string());
            process::exit(0);
        }
    };

    let data = match lib::parse_path(&path) {
        Ok(data) => data,
        Err(err) => {
            println!("{}", err);
            process::exit(0)
        }
    };

    match lib::fetch_data(data).await {
        Err(err) => println!("{}", err.to_string()),
        Ok(_) => (),
    };
}
