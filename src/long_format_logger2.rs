use crate::{BunyanLine, Logger, LogLevel, LoggerOutputConfig};

use std::io::Write;
use std::iter::Iterator;

use httpstatus::StatusCode;

use serde_json::Value;
use serde_json::map::Map as Map;

use itertools::multipeek;

const BASE_INDENT_SIZE: usize = 4;
const LONG_LINE_SIZE: usize = 50;
const DIVIDER: &str = "--";
const REQ_EXTRA: [&str; 7] = ["method", "url", "httpVersion", "body", "header", "headers", "trailers"];
const CLIENT_REQ_EXTRA: [&str; 7] = ["method", "url", "httpVersion", "body", "header", "address", "port"];
const RES_EXTRA: [&str; 4] = ["statusCode", "header", "headers", "trailer"];
const CLIENT_RES_EXTRA: [&str; 5] = ["statusCode", "body", "header", "headers", "trailer"];
const ERR_EXTRA: [&str; 3] = ["message", "name", "stack"];

/// Returns true if the given JSON value is a JSON string and it has a newline character or it is
/// longer than the LONG_LINE_SIZE (50 characterS).
///
/// # Arguments
///
/// * `json_value` - a JSON value of unknown type
fn is_multiline_string(json_value: &Value) -> bool {
    match json_value {
        Value::String(string) => {
            string.contains('\n') || string.len() > LONG_LINE_SIZE
        },
        _ => false
    }
}

/// Returns true if the given JSON value is a JSON object and it is not empty.
///
/// # Arguments
///
/// * `json_value` - a JSON value of unknown type
fn is_object_with_keys(json_value: &Value) -> bool {
    if !json_value.is_object() {
        return false;
    }

    match json_value.as_object() {
        Some(map) => !map.is_empty(),
        None => true
    }
}

/// Returns true if the given JSON value is not a JSON object and or it is an empty JSON object.
///
/// # Arguments
///
/// * `json_value` - a JSON value of unknown type
fn is_empty_object(json_value: &Value) -> bool {
    !is_object_with_keys(json_value)
}

fn write_string_value_params<W: Write>(writer : &mut W, line: &BunyanLine) {
    let other_params = line.other.iter()
        .filter(|&(_, v)| {
            !is_multiline_string(v) && !v.is_array() && is_empty_object(v)
        });
    let mut params = multipeek(other_params);

    let optional_req_id: Option<String> = match line.req_id {
        Some(ref req_id) => {
            match req_id {
                Value::String(req_id_string) => Some(req_id_string.clone()),
                Value::Number(req_id_number) => Some(format!("{}", req_id_number)),
                _ => None
            }
        },
        None => None
    };

    let has_any_params = params.peek().is_some() || optional_req_id.is_some();
    let mut is_first : bool = true;

    if let Some(ref req_id) = optional_req_id {
        is_first = false;
        w!(writer, " (req_id={}", req_id);
    }

    for (k, v) in params {
        if is_first {
            w!(writer, " (");
            is_first = false;
        } else {
            w!(writer, ", ");
        }

        if let Some(param_val) = v.as_str() {
            if param_val.contains(' ') {
                w!(writer, "{}=\"{}\"", k, param_val);
            } else {
                w!(writer, "{}={}", k, param_val);
            }
        } else {
            w!(writer, "{}={}", k, v);
        }
    }

    let had_req_params = write_req_res_string_value_params(
        writer, &line.req, "req", &mut is_first,
        &|k: &str | REQ_EXTRA.contains(&k));
    let had_client_req_params = write_req_res_string_value_params(
        writer, &line.client_req, "client_req", &mut is_first,
        &|k: &str | CLIENT_REQ_EXTRA.contains(&k));
    let had_res_params = write_req_res_string_value_params(
        writer, &line.res, "res",
        &mut is_first, &|k: &str | RES_EXTRA.contains(&k));
    let had_client_res_params = write_req_res_string_value_params(
        writer, &line.client_res, "client_res", &mut is_first,
        &|k: &str | CLIENT_RES_EXTRA.contains(&k));
    let had_err_params = write_req_res_string_value_params(
        writer, &line.err, "err", &mut is_first,
        &|k: &str | ERR_EXTRA.contains(&k));

    if has_any_params || had_req_params || had_client_req_params || had_res_params
        || had_client_res_params || had_err_params {
        w!(writer, ")");
    }
}

fn write_req_res_string_value_params<W: Write>(writer: &mut W,
                                               optional_params: &Option<Map<String, Value>>,
                                               param_name: &str,
                                               is_first: &mut bool,
                                               is_extra_fn: &Fn(&str) -> bool) -> bool {
    fn extra_item_filter(k: &str, v: &Value) -> bool {
        k != "trailer" && (v.is_null() || v.is_boolean())
    }

    match optional_params {
        Some(ref params) => {
            let mut items = multipeek(params.iter()
                .filter(|&(k, v)| {
                    (!is_object_with_keys(v) && !is_extra_fn(k))
                        || (is_extra_fn(k) && extra_item_filter(k,v))
                })
                .map(|t: (&String, &Value)| (format!("{}.{}", param_name, t.0), t.1)));

            if items.peek().is_some() {
                for (k, v) in items {
                    if *is_first {
                        w!(writer, " (");
                        *is_first = false;
                    } else {
                        w!(writer, ", ");
                    }

                    let param_val = string_or_value!(v);

                    let display_key = if k == [param_name, ".raw_body"].concat() {
                        param_name
                    } else {
                        k.as_str()
                    };

                    if param_val.contains(' ') {
                        w!(writer, "{}=\"{}\"", display_key, param_val);
                    } else {
                        w!(writer, "{}={}", display_key, param_val);
                    }
                }

                true
            } else {
                false
            }
        },
        None => false
    }
}

fn write_multiline_string_value_params<W: Write>(writer: &mut W, line: &BunyanLine) -> usize {
    let params = line.other.iter()
        .filter(|&(_, v)| is_multiline_string(v))
        .map(|(k, v)| (k, v.as_str().unwrap_or("undefined")));

    let mut lines_written: usize = 0;

    for (k, v) in params {
        let mut is_first = true;

        for line in v.lines() {
            if is_first {
                wln!(writer, "{:indent$}{}: {}", "", k, line, indent=BASE_INDENT_SIZE);
                is_first = false;
            } else {
                wln!(writer, "{:indent$}{}", "", line, indent=BASE_INDENT_SIZE);
            }
            lines_written += 1;
        }
    }

    lines_written
}

fn write_req<W: Write>(writer: &mut W, optional_req: &Option<Map<String, Value>>) -> (usize, Option<Map<String, Value>>) {
    let mut lines_written: usize = 0;

    lines_written += write_req_summary(writer, optional_req);
    let req_details = write_req_details(writer, optional_req);
    lines_written += req_details.0;

    (lines_written, req_details.1)
}

fn write_client_req<W: Write>(writer: &mut W, optional_req: &Option<Map<String, Value>>) -> (usize, Option<Map<String, Value>>) {
    let mut lines_written: usize = 0;

    if let Some(client_req) = optional_req {
        lines_written += write_req_summary(writer, optional_req);

        if let Some(address_val) = client_req.get("address") {
            if address_val.is_string() {
                w!(writer, "{:indent$}Connecting Host: {}", "", string_or_value!(address_val), indent = BASE_INDENT_SIZE);

                if let Some(port_val) = client_req.get("port") {
                    if port_val.is_string() || port_val.is_number() {
                        w!(writer, ":{}", string_or_value!(port_val));
                    }
                }

                wln!(writer);
                lines_written += 1;
            }
        }

        //         let filter= |k: &String, _: &Value| !CLIENT_REQ_EXTRA.contains(&k.as_str());
        //        lines_written += dump_contents(writer, mut_req_map.clone(), Some("req"), &filter);
        let req_details = write_req_details(writer, optional_req);
        lines_written += req_details.0;

        (lines_written, req_details.1)
    } else {
        (lines_written, None)
    }
}

fn write_req_summary<W: Write>(writer: &mut W, optional_req: &Option<Map<String, Value>>) -> usize {
    let mut lines_written: usize = 0;

    if let Some(ref req_map) = optional_req {
        w!(writer, "{:indent$}", "", indent = BASE_INDENT_SIZE);

        w!(writer, "{} ", get_or_default!(req_map, "method", "undefined"));
        w!(writer, "{} ", get_or_default!(req_map, "url", "undefined"));
        w!(writer, "HTTP/{}", get_or_default!(req_map, "httpVersion", "1.1"));
        wln!(writer);
        lines_written += 1;
    }

    lines_written
}

fn write_req_details<W: Write>(writer: &mut W, optional_req: &Option<Map<String, Value>>) -> (usize, Option<Map<String, Value>>) {
    fn write_keys_and_vals<W: Write>(writer: &mut W, val: &Value) -> usize{
        let mut lines_written: usize = 0;

        if let Some(ref tuples) = val.as_object() {
            for (k, v) in tuples.iter() {
                w!(writer, "{:indent$}{}:", "", k, indent = BASE_INDENT_SIZE);

                let mut is_first = true;

                for line in string_or_value!(v).lines() {
                    if is_first {
                        wln!(writer, " {}", line);
                        is_first = false;
                    } else {
                        wln!(writer, "{:indent$}{}", "", line,
                                     indent = BASE_INDENT_SIZE);
                    }
                    lines_written += 1;
                }
            }
        } else if let Some(ref string_val) = val.as_str() {
            for line in string_val.lines() {
                if line.trim().is_empty() { continue; }

                wln!(writer, "{:indent$}{}", "", line, indent = BASE_INDENT_SIZE);
                lines_written += 1;
            }
        }

        lines_written
    }

    let mut lines_written: usize = 0;

    if let Some(ref req_map) = optional_req {
        let mut_req_map: &mut Map<String, Value> = &mut req_map.clone();

        if let Some(ref header_val) = mut_req_map.remove("header") {
            lines_written += write_keys_and_vals(writer, &header_val);
        }

        if let Some(ref headers_val) = mut_req_map.remove("headers") {
            lines_written += write_keys_and_vals(writer, &headers_val);
        }

        if let Some(ref body) = mut_req_map.remove("body") {
            wln!(writer, "{:indent$}{}", "", string_or_value!(body),
                         indent = BASE_INDENT_SIZE);
            lines_written += 1;
        }

        if let Some(ref trailer_val) = mut_req_map.remove("trailers") {
            lines_written += write_keys_and_vals(writer, &trailer_val);
        }

        for field in CLIENT_REQ_EXTRA.iter() {
            mut_req_map.remove(&field.to_string());
        }

        (lines_written, Some(mut_req_map.clone()))
    } else {
        (lines_written, None)
    }
}

fn write_res<W: Write>(writer: &mut W, optional_res: &Option<Map<String, Value>>) -> usize {
    let mut lines_written: usize = 0;

    if let Some(ref res_map) = optional_res {
        // Unfortunately, we have to match "header" or "headers" to find the headers. If
        // both exist, we throw away the value of "headers" because that's what node-bunyan
        // does.
        let optional_headers: Option<&Value> = match res_map.get("header") {
            Some(header) => Some(header),
            _ => res_map.get("headers")
        };

        if let Some(ref headers) = optional_headers {
            if headers.is_string() {
                let headers_str = headers.as_str().unwrap_or("undefined");

                let http_version = if headers_str.starts_with("HTTP/") {
                    Some(&headers_str[5..8])
                } else {
                    None
                };

                lines_written += write_res_status_code(writer, res_map.get("statusCode"),
                                                       http_version);

                let lines = headers_str.lines();

                for line in lines {
                    if line.is_empty() { continue; }
                    wln!(writer, "{:indent$}{}", "", line, indent = BASE_INDENT_SIZE);
                    lines_written += 1;
                }
            } else if headers.is_object() || headers.is_null() {
                lines_written += write_res_status_code(writer, res_map.get("statusCode"),
                                                       None);
                lines_written += write_headers(writer, &headers);
            }
        } else {
            lines_written += write_res_status_code(writer, res_map.get("statusCode"),
                                                   None);
        }

        if let Some(body_val) = res_map.get("body") {
            if body_val.is_string() {
                let body = string_or_value!(body_val);

                if !body.is_empty() {
                    wln!(writer);
                    lines_written += 1;
                    for line in body.lines() {
                        wln!(writer, "{:indent$}{}", "", line, indent = BASE_INDENT_SIZE);
                        lines_written += 1;
                    }
                }
            }
        }

        for (k, v) in res_map {
            if RES_EXTRA.contains(&k.as_str()) {
                continue;
            }

            // Since empty maps are displayed on top in the first line, we skip them
            if !is_empty_object(v) {
                w!(writer, "{:indent$}res.{}: ", "", k, indent = BASE_INDENT_SIZE);

                lines_written += write_object(writer, v, BASE_INDENT_SIZE);
                wln!(writer);
                lines_written += 1;
            }
        }
    }

    lines_written
}

fn write_res_status_code<W: Write>(writer: &mut W, optional_code: Option<&Value>,
                                   option_http_version: Option<&str>) -> usize {
    let mut lines_written: usize = 0;

    let numeric_status_code = match optional_code {
        Some(json_value) => json_string_or_number_as_u16(json_value),
        None => { None }
    };

    if let Some(code) = numeric_status_code {
        let http_version = option_http_version.unwrap_or("1.1");
        w!(writer, "{:indent$}HTTP/{}", "", http_version, indent = BASE_INDENT_SIZE);

        let status_code = StatusCode::from(code);
        w!(writer, " {} {}", code, status_code.reason_phrase());
        wln!(writer);
        lines_written += 1;
    }

    lines_written
}

fn json_string_or_number_as_u16(val: &Value) -> Option<u16> {
    match val {
        Value::Number(number) => {
            if let Some(code) = number.as_u64() {
                if code > u64::from(std::u16::MAX) {
                    None
                } else {
                    Some(code as u16)
                }
            } else {
                None
            }
        },
        Value::String(string) => {
            let code = string.parse::<u16>();
            match code {
                Ok(val) => Some(val),
                Err(_e) => None
            }
        },
        _ => None
    }
}

fn write_headers<W: Write>(writer: &mut W, headers_val: &Value) -> usize {
    let mut lines_written: usize = 0;

    if let Some(ref headers) = headers_val.as_object() {
        for (k, v) in headers.iter() {
            wln!(writer, "{:indent$}{}: {}", "", k, string_or_value!(v),
                         indent = BASE_INDENT_SIZE);
            lines_written += 1;
        }
    }

    lines_written
}

fn has_object_value_params(line: &BunyanLine) -> bool {
    line.other.iter().any(|(_, v)| v.is_object() || v.is_array())
}

fn write_object_value_params<W: Write>(writer: &mut W, line: &BunyanLine) -> usize {
    let filter = |_: &String, v: &Value| is_object_with_keys(v) || v.is_array();
    dump_contents(writer, line.other.clone(), None, Some(&filter))
}

fn dump_contents<W: Write>(writer: &mut W, map: Map<String, Value>,
                           prefix: Option<&str>,
                           filter: Option<&Fn(&String, &Value) -> bool>) -> usize {
    let params = map.iter();
    let mut lines_written: usize = 0;

    let mut is_first = true;

    for (k, v) in params {
        if let Some(filter_test) = filter {
            if !filter_test(k, v) {
                continue;
            }
        }

        if is_first {
            is_first = false;
        } else {
            wln!(writer, "{:indent$}{}", "", DIVIDER, indent=BASE_INDENT_SIZE);
            lines_written += 1;
        }

        w!(writer, "{:indent$}", "", indent=BASE_INDENT_SIZE);
        if let Some(key_prefix) = prefix {
            w!(writer, "{}.", key_prefix);
        }
        w!(writer, "{}: ", k);

        lines_written += write_object(writer, v,  BASE_INDENT_SIZE);
        wln!(writer);
        lines_written += 1;
    }

    lines_written
}

fn write_object<W: Write>(writer : &mut W, val : &Value, indent: usize) -> usize {
    let mut lines_written: usize = 0;

    match val {
        Value::Null => w!(writer, "null"),
        Value::Bool(boolean) => w!(writer, "{}", boolean),
        Value::Number(number) => w!(writer, "{}", number),
        Value::String(_) => w!(writer, "{}", val),
        Value::Array(array) => {
            lines_written += write_inner_array(writer, array, indent);
        },
        Value::Object(map) => {
            if map.is_empty() {
                w!(writer, "{{}}");
            } else {
                let new_indent = indent + 2;
                let len = map.len();

                wln!(writer, "{{");
                for (pos, (k, v)) in map.iter().enumerate() {
                    w!(writer, "{:indent$}\"{}\": ", "", k, indent=new_indent);
                    lines_written += write_object(writer, v, new_indent);

                    if pos < len - 1 {
                        wln!(writer, ",");
                    } else {
                        wln!(writer);
                    }
                    lines_written += 1;
                }

                w!(writer, "{:indent$}}}", "", indent=indent);
            }
        }
    }

    lines_written
}

fn write_inner_array<W: Write>(writer : &mut W, array : &[Value], indent: usize) -> usize {
    let mut lines_written: usize = 0;

    if array.is_empty() {
        w!(writer, "[]");
        return lines_written
    }

    let new_indent = indent + 2;
    let len = array.len();

    wln!(writer, "[");
    lines_written += 1;

    for (pos, v) in array.iter().enumerate() {
        w!(writer, "{:indent$}", "", indent = new_indent);
        lines_written += write_object(writer, v, new_indent);

        if pos < len - 1 {
            wln!(writer, ",");
        } else {
            wln!(writer);
        }
        lines_written += 1;
    }

    w!(writer, "{:indent$}]", "", indent = indent);

    lines_written
}

fn write_err<W: Write>(writer : &mut W, err_map: &Map<String, Value>) -> usize {
    let mut lines_written = 0;

    if let Some(ref stack_val) = err_map.get("stack") {
        if let Some(ref stack_str) = stack_val.as_str() {
            for line in stack_str.lines() {
                wln!(writer, "{:indent$}{}", "", line, indent=BASE_INDENT_SIZE);
                lines_written += 1;
            }
        } else if let Some(ref stack_array) = stack_val.as_array() {
            for line in stack_array.iter() {
                wln!(writer, "{:indent$}{}", "", string_or_value!(line),
                             indent=BASE_INDENT_SIZE);
                lines_written += 1;
            }
        }
    }

    lines_written
}

impl Logger for BunyanLine {
    fn write_long_format<W: Write>(&self, writer : &mut W, output_config: LoggerOutputConfig) {
        let log_level: LogLevel = self.level.into();
        w!(writer, "[{}] {}: {}/",
               self.time, log_level, self.name);

        if let Some(ref component) = self.component {
            w!(writer, "{}/", component);
        }

        w!(writer, "{} on {}", self.pid, self.hostname);

        if let Some(ref src) = self.src {
            let mut src_written = false;
            if let Some(ref file_val) = src.get("file") {
                if let Some(ref file) = file_val.as_str() {
                    src_written = true;
                    w!(writer, " ({}", file);
                }
            }
            if let Some(ref line_val) = src.get("line") {
                if line_val.is_string() || line_val.is_number() {
                    w!(writer, ":{}", string_or_value!(line_val));
                }
            }
            if let Some(ref func_val) = src.get("func") {
                if func_val.is_string() {
                    w!(writer, " in {}", string_or_value!(func_val));
                }
            }

            if src_written {
                w!(writer, ")");
            }
        }

        w!(writer, ": {}", self.msg);

        write_string_value_params(writer, self);
        wln!(writer);

        let mut needs_divider = false;


        let req_results = if self.req.is_some() {
            if needs_divider {
                wln!(writer, "{:indent$}{}", "", DIVIDER, indent = BASE_INDENT_SIZE);
            }

            let results = write_req(writer, &self.req);
            needs_divider = results.0 > 0;
            results.1
        } else {
            None
        };

        let client_req_results = if self.client_req.is_some() {
            if needs_divider  {
                wln!(writer, "{:indent$}{}", "", DIVIDER, indent = BASE_INDENT_SIZE);
            }

            let results = write_client_req(writer, &self.client_req);
            needs_divider = results.0 > 0;
            results.1
        } else {
            None
        };

        if self.res.is_some() {
            if needs_divider {
                wln!(writer, "{:indent$}{}", "", DIVIDER, indent = BASE_INDENT_SIZE);
            }

            needs_divider = write_res(writer, &self.res) > 0;
        }

        if self.client_res.is_some() {
            if needs_divider {
                wln!(writer, "{:indent$}{}", "", DIVIDER, indent = BASE_INDENT_SIZE);
            }

            needs_divider = write_res(writer, &self.client_res) > 0;
        }

        if let Some(client_req_leftovers) = req_results {
            if needs_divider {
                wln!(writer, "{:indent$}{}", "", DIVIDER, indent = BASE_INDENT_SIZE);
            }

            needs_divider = dump_contents(writer, client_req_leftovers, Some("req"), None) > 0;
        }

        if has_object_value_params(self) {
            if needs_divider {
                wln!(writer, "{:indent$}{}", "", DIVIDER, indent = BASE_INDENT_SIZE);
            }

           needs_divider = write_object_value_params(writer, self) > 0;
        }

        if let Some(ref err_map) = self.err {
            if needs_divider {
                wln!(writer, "{:indent$}{}", "", DIVIDER, indent = BASE_INDENT_SIZE);
            }

            needs_divider = write_err(writer, err_map) > 0;
        }

        if self.other.iter().any(|(_, v)| is_multiline_string(v)) {
            if needs_divider {
                wln!(writer, "{:indent$}{}", "", DIVIDER, indent = BASE_INDENT_SIZE);
            }

            write_multiline_string_value_params(writer, self);
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn multiline_verify_new_line_is_detected() {
        let multiline: Value = Value::from("this\nhas\new lines");
        assert_eq!(is_multiline_string(&multiline), true);
    }

    #[test]
    fn multiline_verify_long_line_is_detected() {
        let multiline: Value = Value::from(format!("{:repeat$}", "-", repeat=LONG_LINE_SIZE + 1));
        assert_eq!(is_multiline_string(&multiline), true);
    }

    #[test]
    fn multiline_verify_no_new_line_is_detected() {
        let multiline: Value = Value::from("this has no new lines");
        assert_eq!(is_multiline_string(&multiline), false);
    }
}
