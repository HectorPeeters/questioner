use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::{self, BufRead};

struct Question {
    text: String,
    code: Option<String>,
    answer: bool,
}

fn main() -> io::Result<()> {
    let matches = App::new("Question randomizer")
        .version("1.0")
        .arg(
            Arg::with_name("input")
                .short("i")
                .help("Starts the application in input mode")
                .takes_value(false),
        )
        .get_matches();

    Ok(())
}
