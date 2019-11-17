#[macro_use]
extern crate serde_derive;
extern crate httpstatus;
extern crate serde;
extern crate serde_json;
extern crate colored;

#[macro_use]
mod macros;
mod divider_writer;
mod errors;
mod long_format_logger;

use crate::errors::LogLevelParseError;

use std::error::Error as StdError;
use std::fmt;
use std::io::{BufRead, Write};
use std::borrow::Cow;

use crate::errors::{Error, Kind, ParseResult};
use serde_json::map::Map;
use serde_json::Error as SerdeError;
use serde_json::Value;
use colored::*;

/// Default indent size in spaces
const BASE_INDENT_SIZE: usize = 4;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
    OTHER(u16),
}

impl LogLevel {
    #[inline]
    pub fn as_u16(&self) -> u16 {
        match *self {
            LogLevel::TRACE => 10,
            LogLevel::DEBUG => 20,
            LogLevel::INFO => 30,
            LogLevel::WARN => 40,
            LogLevel::ERROR => 50,
            LogLevel::FATAL => 60,
            LogLevel::OTHER(code) => code,
        }
    }

    pub fn as_string(&self) -> Cow<'static, str> {
        match *self {
            LogLevel::TRACE => "TRACE".into(),
            LogLevel::DEBUG => "DEBUG".yellow().to_string().into(),
            LogLevel::INFO => "INFO".cyan().to_string().into(),
            LogLevel::WARN => "WARN".magenta().to_string().into(),
            LogLevel::ERROR => "ERROR".red().to_string().into(),
            LogLevel::FATAL => "FATAL".reverse().to_string().into(),
            LogLevel::OTHER(_code) => format!("LVL{}", self.as_u16()).into(),
        }
    }

    pub fn parse<S: Into<String>>(level: S) -> Result<LogLevel, LogLevelParseError> {
        let level = level.into().to_ascii_uppercase();

        match level.as_ref() {
            "TRACE" => Ok(LogLevel::TRACE),
            "DEBUG" => Ok(LogLevel::DEBUG),
            "INFO" => Ok(LogLevel::INFO),
            "WARN" => Ok(LogLevel::WARN),
            "ERROR" => Ok(LogLevel::ERROR),
            "FATAL" => Ok(LogLevel::FATAL),
            _  => {
                let numeric_string = if level.starts_with("LVL") {
                    &level[3..]
                } else {
                    &level
                };

                match numeric_string.parse::<u16>() {
                    Ok(code) => Ok(LogLevel::OTHER(code)),
                    Err(_) => {
                        // Maybe there is a whitespace issue?
                        println!("Attempting to parse numeric string: {}", level);
                        Err(LogLevelParseError::from(level))
                    },
                }
            }

        }
    }
}

impl From<u16> for LogLevel {
    fn from(code: u16) -> Self {
        match code {
            10 => LogLevel::TRACE,
            20 => LogLevel::DEBUG,
            30 => LogLevel::INFO,
            40 => LogLevel::WARN,
            50 => LogLevel::ERROR,
            60 => LogLevel::FATAL,
            _ => LogLevel::OTHER(code),
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let level = self.as_string();

        let left_spaces = if level.len() > 5 { 0 } else { 5 - level.len() };

        write!(f, "{:indent$}{}", "", level, indent = left_spaces)
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct BunyanLine {
    name: String,
    hostname: String,
    pid: usize,
    component: Option<String>,
    level: u16,
    msg: String,
    time: String,
    v: Option<u8>,
    #[serde(flatten)]
    other: Map<String, Value>,
}

pub trait Logger {
    fn write_long_format<W: Write>(
        &self,
        writer: &mut W,
        output_config: &LoggerOutputConfig,
    ) -> ParseResult;
}

pub enum LogFormat {
    Long,
}

pub trait LogWriter {
    fn write_log<W: Write>(
        &self,
        writer: &mut W,
        log: BunyanLine,
        output_config: &LoggerOutputConfig,
    ) -> ParseResult;
}

impl LogWriter for LogFormat {
    fn write_log<W: Write>(
        &self,
        writer: &mut W,
        log: BunyanLine,
        output_config: &LoggerOutputConfig,
    ) -> ParseResult {
        log.write_long_format(writer, output_config)
    }
}

#[derive(Debug, Clone)]
pub struct LoggerOutputConfig {
    pub indent: usize,
    pub is_strict: bool,
    pub is_debug: bool,
    pub level: Option<u16>,
}

fn handle_error<W>(writer: &mut W, error: &Error, output_config: &LoggerOutputConfig)
where
    W: Write,
{
    if !output_config.is_strict || output_config.is_debug {
        let orig_msg = error.to_string();

        let mut split = orig_msg.split(" line ");

        let msg = match split.next() {
            Some(first) => first,
            None => error.description(),
        };

        if output_config.is_debug {
            if let Some(column) = error.column() {
                wln!(
                    std::io::stderr(),
                    "{} on line {} column: {}",
                    msg,
                    error.line_no(),
                    column
                );
            } else {
                wln!(std::io::stderr(), "{} on line {}", msg, error.line_no());
            }
        }

        if !output_config.is_strict {
            wln!(writer, "{}", error.line());
        }
    }
}

pub fn write_bunyan_output<W, R>(
    writer: &mut W,
    reader: R,
    format: &LogFormat,
    output_config: &LoggerOutputConfig,
) where
    W: Write,
    R: BufRead,
{
    let mut line_no: usize = 0;

    reader.lines().for_each(|raw_line| {
        match raw_line {
            Ok(line) => {
                line_no += 1;

                // Don't process empty lines because the output isn't useful to our users
                if !output_config.is_strict && line.trim().is_empty() {
                    wln!(writer);
                } else {
                    let json_result: Result<BunyanLine, SerdeError> = serde_json::from_str(&line);
                    match json_result {
                        Ok(log) => {
                            let write_log = if let Some(output_level) = output_config.level {
                                output_level <= log.level
                            } else {
                                true
                            };

                            if write_log {
                                let result = format.write_log(writer, log, output_config);
                                if let Err(e) = result {
                                    let kind = Kind::from(e);
                                    let error = Error::new(kind, line, line_no, None);
                                    handle_error(writer, &error, &output_config);
                                }
                            }
                        }
                        Err(raw_error) => {
                            let column: usize = raw_error.column();
                            let kind = Kind::from(raw_error);
                            let error = Error::new(kind, line, line_no, Some(column));
                            handle_error(writer, &error, &output_config);
                        }
                    }
                }
            }
            Err(e) => {
                panic!(e);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_to_known_log_level() {
        let levels = vec![
            LogLevel::TRACE,
            LogLevel::DEBUG,
            LogLevel::INFO,
            LogLevel::ERROR,
            LogLevel::FATAL,
        ];
        assert_log_levels_parse(levels);
    }

    #[test]
    fn can_parse_custom_level_log_level() {
        let levels = vec![
            LogLevel::OTHER(0),
            LogLevel::OTHER(100),
            LogLevel::OTHER(1001),
        ];
        assert_log_levels_parse(levels);
    }

    fn assert_log_levels_parse(levels: Vec<LogLevel>) {
        for test_level in levels {
            let level_string = test_level.as_string();
            let lower_case_level_string = level_string.to_ascii_lowercase();

            println!("Attempting to parse string [{}] as log level literal", level_string);

            // test parsing uppercase
            match LogLevel::parse(level_string) {
                Ok(level) => assert_eq!(level, test_level, "Unable to parse input to log level"),
                Err(err) => panic!(err),
            }

            // test parsing lowercase
            match LogLevel::parse(lower_case_level_string) {
                Ok(level) => assert_eq!(level, test_level, "Unable to parse input to log level"),
                Err(err) => panic!(err),
            }
        }
    }
}
