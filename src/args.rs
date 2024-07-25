use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Download GitHub directories/files",
    help_template = "\
{before-help}{name} {version}
{about-with-newline}
{usage-heading}\n  {usage}

{all-args}{after-help}
")]
pub struct CommandArgs {
    ///  URLs for GitHub directories or files. You can pass a single URL or multiple comma-delimited URLs
    #[arg(
        value_delimiter = ',',
        action = ArgAction::Set, 
        num_args = 1, 
    )]
    pub urls: Vec<String>,

    #[arg()]
    pub path: Option<String>,

    /// Download and zip directories
    #[arg(short = 'z', long = "zip")]
    pub zipped: bool,
}
