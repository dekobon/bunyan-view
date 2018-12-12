#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate httpstatus;
extern crate itertools;

#[macro_use]
mod macros;
mod long_format_logger;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, Write};

use serde_json::Value;
use serde_json::map::Map as Map;
use serde_json::Error as SerdeError;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
    OTHER(u16)
}

impl LogLevel {
    #[inline]
    pub fn as_u16(&self) -> u16 {
        match *self {
            LogLevel::TRACE       => 10,
            LogLevel::DEBUG       => 20,
            LogLevel::INFO        => 30,
            LogLevel::WARN        => 40,
            LogLevel::ERROR       => 50,
            LogLevel::FATAL       => 60,
            LogLevel::OTHER(code) => code
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        match *self {
            LogLevel::TRACE        => "TRACE",
            LogLevel::DEBUG        => "DEBUG",
            LogLevel::INFO         => "INFO",
            LogLevel::WARN         => "WARN",
            LogLevel::ERROR        => "ERROR",
            LogLevel::FATAL        => "FATAL",
            LogLevel::OTHER(_code) => "LVL"
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
            _  => LogLevel::OTHER(code)
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let level = if let LogLevel::OTHER(_) = *self {
            format!("{}{}", self.as_str(), self.as_u16())
        } else {
            self.as_str().to_string()
        };

        let left_spaces = if level.len() > 5 {
            0
        } else {
            5 - level.len()
        };

        write!(f, "{:indent$}{}", "", level, indent=left_spaces)
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RawBunyanLine {
    name: String,
    hostname: String,
    pid: usize,
    component: Option<String>,
    level: u16,
    msg: String,
    time: String,
    v: Option<u8>,
    req_id: Option<Value>,
    req: Option<Map<String, Value>>,
    client_req: Option<Map<String, Value>>,
    client_res: Option<Value>,
    res: Option<Value>,
    err: Option<Map<String, Value>>,
    src: Option<Value>,
    #[serde(flatten)]
    other: Map<String, Value>
}

#[derive(Debug)]
pub struct BunyanLine {
    name: String,
    hostname: String,
    pid: usize,
    component: Option<String>,
    level: u16,
    msg: String,
    time: String,
    v: Option<u8>,
    req_id: Option<Value>,
    req: Option<Map<String, Value>>,
    client_req: Option<Map<String, Value>>,
    client_res: Option<Map<String, Value>>,
    res: Option<Map<String, Value>>,
    err: Option<Map<String, Value>>,
    src: Option<Map<String, Value>>,
    other: Map<String, Value>
}

pub trait Logger {
    fn write_long_format<W: Write>(&self, writer : &mut W);
}

pub enum LogFormat {
    Long
}

pub trait LogWriter {
    fn write_log<W: Write>(&self, writer: &mut W, log: BunyanLine, indent: Option<usize>);
}

impl LogWriter for LogFormat {
    fn write_log<W: Write>(&self, writer: &mut W, log: BunyanLine, indent: Option<usize>) {
        log.write_long_format(writer)
    }
}

fn convert_value_to_map(val: &Value) -> Map<String, Value> {
    if val.is_object() {
        val.as_object().unwrap().clone()
    } else {
        let mut map = Map::new();
        let s: String = if val.is_string() {
            let raw_s = val.to_string();
            if raw_s.len() > 1 {
                raw_s[1..raw_s.len() - 1].to_string()
            } else {
                raw_s
            }
        } else {
            val.to_string()
        };

        map.insert("raw_body".to_string(), Value::from(s));
        map
    }
}

fn convert_from_raw_serialized_format(raw: RawBunyanLine) -> BunyanLine {
    let other = raw.other;

    let client_res = match raw.client_res {
        Some(cr) => Some(convert_value_to_map(&cr)),
        None => None
    };

    let res = match raw.res {
        Some(r) => {
            let map = convert_value_to_map(&r);
            Some(map)
        },
        None => None
    };

    let src = match raw.src {
        Some(src_val) => {
            if let Some(src_map) = src_val.as_object() {
                Some(src_map.clone())
            } else if let Some(src_str) = src_val.as_str() {
                let mut map = Map::new();
                map.insert("file".to_string(), Value::from(src_str));
                Some(map)
            } else {
                None
            }
        }
        None => None
    };

    BunyanLine {
        name: raw.name,
        hostname: raw.hostname,
        pid: raw.pid,
        component: raw.component,
        level: raw.level,
        msg: raw.msg,
        time: raw.time,
        v: raw.v,
        req_id: raw.req_id,
        req: raw.req,
        client_req: raw.client_req,
        client_res,
        res,
        err: raw.err,
        src,
        other
    }
}

pub fn write_bunyan_output<W, R>(writer: &mut W, reader: R, format: &LogFormat,
    is_strict: bool, is_debug: bool, indent: Option<usize>)
    where W: Write,
    R: BufRead
{
    let mut line_no: usize = 0;

    reader.lines().for_each(|raw_line| {
        match raw_line {
            Ok(line) => {
                line_no += 1;

                // Don't process empty lines because the output isn't useful to our users
                if !is_strict && line.trim().is_empty() {
                    wln!(writer);
                } else {
                    let json_result: Result<RawBunyanLine, SerdeError> = serde_json::from_str(&line);
                    match json_result {
                        Ok(raw) => {
                            let log = convert_from_raw_serialized_format(raw);
                            format.write_log(writer, log, indent)
                        },
                        Err(e) => {
                            if !is_strict || is_debug {
                                let orig_msg = e.to_string().clone();
                                let mut split = orig_msg.split(" line ");

                                let msg = match split.next() {
                                    Some(first) => first,
                                    None => e.description()
                                };

                                if is_debug {
                                    wln!(std::io::stderr(), "{} line {} column: {}",
                                             msg, line_no, e.column());
                                }

                                if !is_strict {
                                    wln!(writer, "{}", line);
                                }
                            }
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
