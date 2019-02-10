#[macro_use]
extern crate clap;

extern crate bunyan_view;
extern crate flate2;

use bunyan_view::{LogFormat, LogLevel, LoggerOutputConfig};
use clap::{App, AppSettings, ArgMatches, Arg};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let matches = App::new("Bunyan View")
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::TrailingVarArg)
        .version(crate_version!())
        .author(crate_authors!())
        .about("Displays bunyan format log files to the console")
        .after_help("Environment Variables:\n  BUNYAN_NO_COLOR    Set to a non-empty value to force no output coloring. See \"--no-color\".")
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
        .arg(Arg::with_name("color")
            .help("Force coloring even if terminal doesn't support it")
            .long("color")
            .takes_value(false)
            .required(false))
        .arg(Arg::with_name("no-color")
            .help("Force no coloring (e.g. terminal doesn't support it)")
            .long("no-color")
            .takes_value(false)
            .required(false))
        .arg(Arg::with_name("FILE")
            .help("Sets the input file(s) to use")
            .required(false)
            .multiple(true)
            .index(1))
        .get_matches();

    let level: Option<u16> = match matches.value_of("level") {
        Some(level_string) => match LogLevel::parse(level_string) {
            Ok(level) => Some(level.as_u16()),
            Err(e) => {
                eprintln!("{}: {}", e, level_string);
                std::process::exit(1);
            }
        },
        None => None,
    };

    let output_config = LoggerOutputConfig {
        indent: 4,
        is_strict: matches.is_present("strict"),
        is_debug: matches.is_present("debug"),
        level,
    };

    apply_color_settings(&matches);

    match matches.values_of("FILE") {
        Some(filenames) => {
            for filename in filenames {
                let file_result = File::open(filename);

                match file_result {
                    Ok(_) => {}
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

                bunyan_view::write_bunyan_output(
                    &mut std::io::stdout(),
                    reader,
                    &LogFormat::Long,
                    &output_config,
                );
            }
        }
        None => {
            let reader = Box::new(BufReader::new(std::io::stdin()));
            bunyan_view::write_bunyan_output(
                &mut std::io::stdout(),
                reader,
                &LogFormat::Long,
                &output_config,
            );
        }
    }
}

/// Reads the CLI parameters and environment variables set upon execution and selectively
/// enables or disables color support
///
/// # Arguments
/// * `matches` - CLAP flags data structure
fn apply_color_settings(matches: &ArgMatches) {
    if matches.is_present("color") && matches.is_present("no-color") {
        eprintln!("ERROR: Contradictory color settings: use --no-color OR --color");
        std::process::exit(1);
    }

    // If BUNYAN_NO_COLOR is set, we intentionally ignore the --color setting
    if matches.is_present("no-color") || ::std::env::var_os("BUNYAN_NO_COLOR").is_some() {
        colored::control::set_override(false);
    } else if matches.is_present("color") {
        colored::control::set_override(true);
    }
}
