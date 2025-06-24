use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug)]
pub struct Config {
    input_file: String,
    output_file: Option<String>,
    count: bool,
}

type UniqResult<T> = Result<T, Box<dyn Error>>;

const STDIN_FILE: &str = "-";

pub fn get_args() -> UniqResult<Config> {
    let matches = App::new("uniq")
        .version("0.1.0")
        .author("ryanplusplus")
        .about("uniq, but Rust")
        .arg(
            Arg::with_name("input_file")
                .value_name("IN_FILE")
                .help("Input file.")
                .default_value(STDIN_FILE),
        )
        .arg(
            Arg::with_name("output_file")
                .value_name("OUT_FILE")
                .help("Output file."),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("Precede each output line with the count of the number of times the line occurred in the input, followed by a single space.")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        input_file: matches.value_of_lossy("input_file").unwrap().to_string(),
        output_file: matches.value_of("output_file").map(String::from),
        count: matches.is_present("count"),
    })
}

fn open_for_reading(filename: &str) -> UniqResult<Box<dyn BufRead>> {
    match filename {
        STDIN_FILE => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn open_for_writing(filename: &Option<String>) -> UniqResult<Box<dyn Write>> {
    match filename {
        Some(filename) => Ok(Box::new(File::create(filename)?)),
        None => Ok(Box::new(io::stdout())),
    }
}

pub fn run(config: Config) -> UniqResult<()> {
    let mut input = open_for_reading(&config.input_file)
        .map_err(|e| format!("{}: {}", config.input_file, e))?;
    let mut output = open_for_writing(&config.output_file)?;

    let mut write = |count: u64, s: &str| -> UniqResult<()> {
        if config.count {
            write!(output, "{:>4} {}", count, s)?;
        } else {
            write!(output, "{}", s)?;
        }
        Ok(())
    };

    let mut last: Option<String> = None;
    let mut count: u64 = 0;
    let mut line = String::new();

    loop {
        let bytes = input.read_line(&mut line)?;

        if bytes == 0 {
            break;
        }

        if let Some(ref some_last) = last {
            if line.trim_end() != some_last.trim_end() {
                write(count, some_last)?;
                last = Some(line.clone());
                count = 0;
            }
        } else {
            last = Some(line.clone());
        }

        count += 1;
        line.clear();
    }

    if count > 0 {
        write(count, &last.unwrap())?;
    }

    Ok(())
}
