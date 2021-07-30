use clap::{load_yaml, App};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let url = matches.value_of("URL");

    println!("{}", url.unwrap());
}
