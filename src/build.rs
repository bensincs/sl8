mod cli;

use std::{path::Path, fs::File, io::Write};

use clap_markdown::help_markdown_command;
use cli::cli;

fn main() {
    let cmd = cli();

    let md = help_markdown_command(&cmd);

    File::create(Path::new("CLI_HELP.md"))
        .unwrap()
        .write_all(md.as_bytes())
        .unwrap();
}