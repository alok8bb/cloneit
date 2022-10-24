use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(
    name = "cloneit",
    author = "Alok P",
    version = "2.0.0",
    about = "Download GitHub directories/files",
    help_template = "\
{before-help}{name} {version}
{about-with-newline}
{usage-heading}\n  {usage}

{all-args}{after-help}
")]
pub struct CommandArgs {
    #[arg(
        value_delimiter = ',',
        action = ArgAction::Set, 
        num_args = 1, 
        help = "URL to the GitHub directory or file. You can pass a single URL or multiple comma-delimited URLs"
    )]
    pub urls: Vec<String>,

    #[arg()]
    pub path: Option<String>,

    #[arg(short = 'z', long = "zip", help = "download and zip directory")]
    pub zipped: bool,
}
