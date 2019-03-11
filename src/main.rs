#[macro_use]
extern crate clap;
extern crate bunyan_view;
extern crate flate2;
extern crate pager;

use std::env;
use std::path::PathBuf;
use bunyan_view::{LogFormat, LogLevel, LoggerOutputConfig};
use clap::{App, AppSettings, ArgMatches, Arg};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use pager::Pager;

fn main() {
    let env_var_help = "Environment Variables:
  BUNYAN_NO_COLOR    Set to a non-empty value to force no output coloring. See \"--no-color\".
  BUNYAN_NO_PAGER    Disable piping output to a pager. See \"--no-pager\".";

    let matches = App::new("Bunyan View")
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::TrailingVarArg)
        .version(crate_version!())
        .author(crate_authors!())
        .about("Displays bunyan format log files to the console")
        .after_help(env_var_help)
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
        .arg(Arg::with_name("PID")
            .help("Runtime log snooping (via DTrace, only on supported platforms). Process bunyan:log-* probes from the process with the given PID. Can be used multiple times, or specify all processes with \"*\", or a set of processes whose command & args match a pattern with \"-p NAME\".")
            .long("pids")
            .short("p")
            .takes_value(true)
            .required(false)
            .multiple(true))
        .arg(Arg::with_name("level")
            .help("Only show messages at or above the specified level. You can specify level *names* or the internal numeric values.")
            .long("level")
            .short("l")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("pager")
            .help("Pipe output into `less` (or $PAGER if set), if stdout is a TTY. This overrides $BUNYAN_NO_PAGER.")
            .long("pager")
            .takes_value(false)
            .required(false))
        .arg(Arg::with_name("no-pager")
            .help("Do not pipe output into a pager.")
            .long("no-pager")
            .takes_value(false)
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

    // If no pids or files are specified, we assume STDIN
    if !matches.is_present("FILE") && !matches.is_present("PID") {
        let reader = Box::new(BufReader::new(std::io::stdin()));
        bunyan_view::write_bunyan_output(
            &mut std::io::stdout(),
            reader,
            &LogFormat::Long,
            &output_config,
        );
    }

    if let Some(pids) = matches.values_of("PID") {

    }

    if let Some(filenames) = matches.values_of("FILE") {
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

            // We only enable pager support when a file has been directly specified
            apply_pager_settings(&matches);

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
}

/// Reads the CLI parameters and environment variables set upon execution and selectively
/// enables or disables pager support
///
/// # Arguments
/// * `matches` - CLAP flags data structure
fn apply_pager_settings(matches: &ArgMatches) {
    if matches.is_present("no-pager") && matches.is_present("pager") {
        eprintln!("ERROR: Contradictory pager settings: use --no-pager OR --pager");
        std::process::exit(1);
    }

    // If BUNYAN_NO_PAGER is set, we intentionally ignore the --pager setting
    // The default setting is to enable the pager if we were provided a file
    if !matches.is_present("no-pager") && ::std::env::var_os("BUNYAN_NO_PAGER").is_none() {
        Pager::new().setup();
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

/// Finds the dtrace executable within the system PATH environment variable.
fn find_dtrace() -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let dtrace_path = dir.join("dtrace");
            if dtrace_path.is_file() {
                Some(dtrace_path)
            } else {
                None
            }
        }).next()
    })
}

fn spawn_dtrace(pids: clap::Values) {
    let dtrace_path = find_dtrace();
    if dtrace_path.is_none() {
        eprintln!("DTrace could not be found in path");
        std::process::exit(1);
    }

    let mut child = std::process::Command::new(dtrace_path.unwrap());
}