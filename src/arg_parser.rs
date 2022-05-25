use clap::{Arg, Command};

#[must_use]
pub fn create_args_parser<'a>() -> Command<'a> {
    Command::new("cloneit")
        .version("1.0.0")
        .author("Alok P <alok8bb@gmail.com>")
        .about("Download specific GitHub directories or files")
        .args(&[
            Arg::new("url")
                .long_help(
                    "URL to the GitHub directory or file. You can pass a single URL or multiple comma-delimited URLs e.g.
                    https://github.com/fpic/linpeace.py,https://github.com/s0xf/r2gihdra.c,https://github.com/fpic/defpol/master")
                .required(true)
                .takes_value(true)
                .multiple_values(true)
                .use_value_delimiter(true)
                .require_value_delimiter(true),
            Arg::new("zip")
                .short('z')
                .multiple_occurrences(false)
                .help("Download zipped directory"),
            Arg::new("link")
                .short('l')
                .multiple_occurrences(false)
                .help("Generate download link to zipped file"),
        ])
}
