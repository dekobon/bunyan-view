#[macro_use]
extern crate clap;

extern crate bunyan_view;
extern crate flate2;

use std::fs::File;
use std::io::{BufReader, BufRead};
use flate2::read::GzDecoder;
use bunyan_view::{LogFormat, LogLevel, LoggerOutputConfig};
use clap::{Arg, App, AppSettings};

fn main() {
    let matches = App::new("Bunyan View")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::TrailingVarArg)
        .version(crate_version!())
        .author(crate_authors!())
        .about("Displays bunyan format log files to the console")
        .arg(Arg::with_name("debug")
            .help("Display deserialization errors and expectation mismatches to STDERR.")
            .long("debug")
            .short("d")
            .takes_value(false)
            .required(false))
        .arg(Arg::with_name("strict")
            .help("Suppress all but legal Bunyan JSON log lines. By default non-JSON, and non-Bunyan lines are passed through.")
            .long("strict")
            .takes_value(false)
            .required(false))
        .arg(Arg::with_name("level")
            .help("Only show messages at or above the specified level. You can specify level *names* or the internal numeric values.")
            .long("level")
            .short("l")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("FILE")
            .help("Sets the input file(s) to use")
            .required(false)
            .multiple(true)
            .index(1))
        .get_matches();

    let level: Option<u16> = match matches.value_of("level") {
        Some(level_string) => {
            match LogLevel::parse(level_string) {
                Ok(level) => Some(level.as_u16()),
                Err(e) => {
                    eprintln!("{}: {}", e, level_string);
                    std::process::exit(1);
                }
            }
        }
        None => None
    };

    let output_config = LoggerOutputConfig {
        indent: 4,
        is_strict: matches.is_present("strict"),
        is_debug: matches.is_present("debug"),
        level
    };

    match matches.values_of("FILE") {
        Some(filenames) => {
            for filename in filenames {
                let file_result = File::open(filename);

                match file_result {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("{}: {}", e, filename);
                        std::process::exit(1);
                    }
                }

                let file = file_result.unwrap();

                let reader: Box<BufRead> = if filename.ends_with(".gz") {
                    Box::new(BufReader::new(GzDecoder::new(BufReader::new(file))))
                } else {
                    Box::new(BufReader::new(file))
                };

                bunyan_view::write_bunyan_output(&mut std::io::stdout(), reader,
                                                 &LogFormat::Long, output_config.clone());
            }
        },
        None => {
            let reader = Box::new(BufReader::new(std::io::stdin()));
            bunyan_view::write_bunyan_output(&mut std::io::stdout(), reader,
                                             &LogFormat::Long, output_config);
        }
    }
}
