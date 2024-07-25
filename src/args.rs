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
    /// URLs for GitHub directories or files to download. You can pass a single URL or multiple comma-delimited URLs
    #[arg(
        value_delimiter = ',',
        action = ArgAction::Set, 
        num_args(1..), 
        required = true,
    )]
    pub urls: Vec<String>,

    #[arg(short, long)]
    pub path: Option<String>,

    /// Download and zip directories
    #[arg(short, long = "zip")]
    pub zipped: bool,
}
