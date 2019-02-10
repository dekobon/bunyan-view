use crate::divider_writer::DividerWriter;
use crate::errors::{BunyanLogParseError, ParseIntFromJsonError, ParseResult};
use crate::BASE_INDENT_SIZE;
use crate::{BunyanLine, LogLevel, Logger, LoggerOutputConfig};

use std::io::Write;

use httpstatus::StatusCode;

use serde_json::map::Map;
use serde_json::Value;

use colored::*;

/// Maximum characters for a string value in the extra parameters section
const LONG_LINE_SIZE: usize = 50;
/// Reserved keywords for requests records
const REQ_RESERVED: [&str; 6] = [
    "method",
    "url",
    "httpVersion",
    "body",
    "headers",
    "trailers",
];
/// Reserved keywords for client requests records
const CLIENT_REQ_RESERVED: [&str; 8] = [
    "method",
    "url",
    "httpVersion",
    "body",
    "headers",
    "trailers",
    "address",
    "port",
];
/// Reserved keywords for responses records
const RES_RESERVED: [&str; 6] = [
    "statusCode",
    "header",
    "headers",
    "trailer",
    "body",
    "trailer",
];
/// Reserved keywords for client responses records
const CLIENT_RES_RESERVED: [&str; 5] = ["statusCode", "body", "header", "headers", "trailer"];
/// Reserved keywords for error records
const ERR_RESERVED: [&str; 3] = ["message", "name", "stack"];
/// Reserved keywords for the `other` map in `BunyanLine`
const GENERAL_RESERVED: [&str; 5] = ["req", "client_req", "res", "client_res", "err"];
/// Default assumed HTTP version
const DEFAULT_HTTP_VERSION: &str = "1.1";

/// Writes the src information of the log line if it is present.
///
/// # Arguments
///
/// * `writer` - Write implementation to output data to
/// * `other` - Mutable map containing JSON optional JSON data. Keys will be removed as processed.
///
fn write_src<W: Write>(writer: &mut W, other: &mut Map<String, Value>) {
    if let Some(ref src) = other.remove("src") {
        match src {
            Value::Object(map) => {
                // We only display the src information if [src.file] is present
                if let Some(ref file) = map.get("file") {
                    w!(writer, "{}", " (".green());
                    w!(writer, "{}", string_or_value!(file).green());

                    if let Some(ref line) = map.get("line") {
                        w!(writer, ":{}", string_or_value!(line).green());
                    }

                    if let Some(ref func) = map.get("func") {
                        w!(writer, "{}", " in ");
                        w!(writer, "{}", string_or_value!(func).bright_green());
                    }

                    w!(writer, "{}", ")".green());
                }
            }
            Value::String(text) => w!(writer, " ({})", text),
            _ => (),
        }
    }
}

/// Writes all of the extra parameters to the top line of output by iterating through the `others`
/// map provided.
///
/// # Arguments
/// * `writer` - Write implementation to output data to
/// * `other` - Map containing all non-explicitly deserialized keys and values
/// * `details` - Mutable vector containing strings to be written as output later
///
fn write_all_extra_params<W: Write>(
    writer: &mut W,
    other: &mut Map<String, Value>,
    details: &mut Vec<String>,
) {
    /// Returns the passed value as a pretty printed JSON string with indents.
    ///
    /// # Arguments
    /// * `key` - Key associated with value being processed
    /// * `value` - Value to be converted to a pretty printed string
    /// * `caller_option` - Optional name of top-level record (eg `req`, `res`, `err`, etc)
    ///
    fn detail_pretty_print(key: &str, value: &Value, caller_option: Option<&str>) -> String {
        let pretty = ::serde_json::to_string_pretty(value).unwrap_or("[malformed]".to_string());

        match caller_option {
            Some(caller) => format!("{}.{}: {}", caller, key, pretty),
            None => format!("{}: {}", key, pretty),
        }
    }

    /// Returns true if the given JSON value is a JSON string and it has a newline character or it
    /// is longer than the LONG_LINE_SIZE (50 characterS).
    ///
    /// # Arguments
    ///
    /// * `text` - string to test to see if it qualifies for multiline output
    fn is_multiline_string(text: &str) -> bool {
        text.contains('\n') || text.len() > LONG_LINE_SIZE
    }

    /// Returns the passed value as a string optionally with enclosing quotes. If
    /// the conversion to a string of the value yields a string with spaces, then
    /// the returned string will be enclosed with double quotes.
    ///
    /// # Arguments
    ///
    /// * `value` - a serde JSON value object to convert to a String
    fn quoteify(value: &Value) -> String {
        if let Some(text) = value.as_str() {
            if text.contains(' ') {
                return value.to_string();
            }
        }

        string_or_value!(value)
    }

    /// Returns an optional string representing the string presentation of an extra parameter. When
    /// a `None` value is returned, the value has been added to the `details` vector.
    ///
    /// # Arguments
    /// * `key` - Key associated with value being processed
    /// * `value` - Value to be converted to a pretty printed string
    /// * `caller_option` - Optional name of top-level record (eg `req`, `res`, `err`, etc)
    /// * `details` - Mutable vector containing strings to be written as output later
    fn stringify(
        key: &str,
        value: &Value,
        caller_option: Option<&str>,
        details: &mut Vec<String>,
    ) -> Option<String> {
        match value {
            Value::String(text) => {
                // Add long strings to details
                if is_multiline_string(text) {
                    let detail = match caller_option {
                        Some(caller) => format!("{}.{}: {}", caller, key, text),
                        None => format!("{}: {}", key, text),
                    };

                    details.push(detail);

                    None
                // Wrap strings with spaces in quotation marks
                } else {
                    Some(quoteify(value))
                }
            }
            Value::Number(_) => Some(string_or_value!(value)),
            Value::Bool(_) => Some(string_or_value!(value)),
            Value::Null => Some(string_or_value!(value)),
            Value::Object(map) => {
                if map.is_empty() {
                    Some("{}".to_string())
                } else {
                    details.push(detail_pretty_print(key, value, caller_option));
                    None
                }
            }
            Value::Array(array) => {
                if array.is_empty() {
                    Some("[]".to_string())
                } else {
                    details.push(detail_pretty_print(key, value, caller_option));
                    None
                }
            }
        }
    }

    /// Writes the loading open parentheses if `if_first` is true. Otherwise, if
    /// `is_first` is false, then it writes a leading command and space.
    ///
    /// # Arguments
    ///
    /// * `writer` - Write implementation to output data to
    /// * `is_first` - Mutable boolean indicating if the first parameter has been processed
    fn write_formatting<W: Write>(writer: &mut W, is_first: &mut bool) {
        if *is_first {
            w!(writer, " (");
            *is_first = false;
        } else {
            w!(writer, ", ");
        }
    }

    /// Writes the extra parameters for the passed `optional_node` value if not `None`.
    ///
    /// # Arguments
    /// * `writer` - Write implementation to output data to
    /// * `caller_option` - Optional name of top-level record (eg `req`, `res`, `err`, etc)
    /// * `is_first` - Mutable boolean indicating if the first parameter has been processed
    /// * `optional_node` - Optional Json object represented as `Value` containing parameters to be processed
    /// * `details` - Mutable vector containing strings to be written as output later
    /// * `exclude` - Closure in which when evaluated is true will exclude a given parameter
    ///
    fn write_params_for_object<W: Write>(
        writer: &mut W,
        caller_option: Option<&str>,
        is_first: &mut bool,
        node_option: Option<&Value>,
        details: &mut Vec<String>,
        exclude: &Fn(&str) -> bool,
    ) {
        if node_option.is_none() {
            return;
        }

        let node = node_option.unwrap();

        // Display strings, numbers and null values, as-is
        if caller_option.is_some()
            && (node.is_string() || node.is_number() || node.is_null() || node.is_boolean())
        {
            write_formatting(writer, is_first);
            w!(writer, "{}={}", caller_option.unwrap(), quoteify(node));
            return;
        }

        if caller_option.is_some() && node.is_array() {
            let value = stringify(caller_option.unwrap(), node, None, details);
            if let Some(text) = value {
                write_formatting(writer, is_first);
                w!(writer, "{}={}\n", caller_option.unwrap(), text);
            }
            return;
        }

        let map = node.as_object().unwrap();

        for (k, v) in map.iter() {
            if exclude(&k.as_str()) {
                continue;
            }

            let value: Option<String> = stringify(k, v, caller_option, details);

            if let Some(text) = value {
                write_formatting(writer, is_first);

                match caller_option {
                    Some(caller) => w!(writer, "{}.{}={}", caller, k, text),
                    None => w!(writer, "{}={}", k, text),
                }
            }
        }
    }

    let mut is_first: bool = true;

    // REQUEST ID [req_id] - special case we always write this first for visibility
    if let Some(req_id) = other.remove("req_id") {
        write_formatting(writer, &mut is_first);
        w!(writer, "req_id={}", string_or_value!(req_id));
    }

    /* Note: based on logic in write_params_for_object, parameters that do not fit
    properly within the extra parameters section, will be added to the details
    vector. */

    // Write out all keys and values that is are in GENERAL_RESERVED.
    let other_value = ::serde_json::to_value(&other).unwrap();
    write_params_for_object(
        writer,
        None,
        &mut is_first,
        Some(&other_value),
        details,
        &|k: &str| GENERAL_RESERVED.contains(&k),
    );

    /* Below, we write out the parameters of all JSON keys that are present in
    GENERAL_RESERVED in order to output any of the contents that need to be
    present in the extra parameters section of the logs. */

    // REQUEST [rec]
    write_params_for_object(
        writer,
        Some("req"),
        &mut is_first,
        other.get("req"),
        details,
        &|k: &str| REQ_RESERVED.contains(&k),
    );

    // CLIENT REQUEST [client_req]
    write_params_for_object(
        writer,
        Some("client_req"),
        &mut is_first,
        other.get("client_req"),
        details,
        &|k: &str| CLIENT_REQ_RESERVED.contains(&k),
    );

    // RESPONSE [res]
    write_params_for_object(
        writer,
        Some("res"),
        &mut is_first,
        other.get("res"),
        details,
        &|k: &str| RES_RESERVED.contains(&k),
    );

    // CLIENT RESPONSE [client_res]
    write_params_for_object(
        writer,
        Some("client_res"),
        &mut is_first,
        other.get("client_res"),
        details,
        &|k: &str| CLIENT_RES_RESERVED.contains(&k),
    );

    // ERROR INFORMATION [err]
    write_params_for_object(
        writer,
        Some("err"),
        &mut is_first,
        other.get("err"),
        details,
        &|k: &str| ERR_RESERVED.contains(&k),
    );

    if !is_first {
        w!(writer, ")");
    }
}

/// Writes the HTTP request information logged for the line.
///
/// # Arguments
///
/// * `writer` - Write implementation to output data to
/// * `other` - Map containing all non-explicitly deserialized keys and values
///
fn write_req<W: Write>(writer: &mut W, key: &str, other: &mut Map<String, Value>) {
    /// Writes the method, url and HTTP version associated with a request.
    ///
    /// # Arguments
    ///
    /// * `writer` - Write implementation to output data to
    /// * `caller_name` - text indicating if we have been invoked from a "req" or "client_req" code path
    /// * `req_map` - Mutable map request data. Keys will be removed as processed.
    ///
    /// # Errors
    ///
    /// This function will return None if no errors have been encountered. In the case of parsing logic
    /// errors where the JSON data is not in the expected format, it will return a
    /// `ParseResult`.
    ///
    fn write_req_summary<W: Write>(
        writer: &mut W,
        caller: &str,
        req_map: &mut Map<String, Value>,
    ) -> ParseResult {
        w!(writer, "{:indent$}", "", indent = BASE_INDENT_SIZE);

        if let Some(method) = req_map.remove("method") {
            if let Some(method_text) = method.as_str() {
                w!(writer, "{} ", method_text.yellow());
            } else {
                return Err(BunyanLogParseError::new(format!(
                    "[{}.method] is not a JSON string",
                    caller
                )));
            }
        } else {
            return Err(BunyanLogParseError::new(format!(
                "[{}.method] is not present",
                caller
            )));
        }

        if let Some(url) = req_map.remove("url") {
            if let Some(url_text) = url.as_str() {
                w!(writer, "{} ", url_text.bright_blue());
            } else {
                return Err(BunyanLogParseError::new(format!(
                    "[{}.url] is not a JSON string",
                    caller
                )));
            }
        } else {
            return Err(BunyanLogParseError::new(format!(
                "[{}.url] is not present",
                caller
            )));
        }

        if let Some(http_version) = req_map.remove("httpVersion") {
            match http_version {
                Value::String(text) => w!(writer, "HTTP/{}", text),
                Value::Number(number) => w!(writer, "HTTP/{}", number),
                Value::Null => w!(writer, "HTTP/{}", DEFAULT_HTTP_VERSION),
                _ => {
                    return Err(BunyanLogParseError::new(format!(
                        "[{}.httpVersion] is not a string or number",
                        caller
                    )))
                }
            };
        } else {
            // we default to 1.1 if value is not present because that's what node bunyan does
            w!(writer, "HTTP/{}", DEFAULT_HTTP_VERSION);
        }

        wln!(writer);
        Ok(())
    }

    let req_option = other.remove(key);

    if req_option.is_none() {
        return;
    }

    let mut req = req_option.unwrap();

    if !req.is_object() {
        return;
    }

    let req_map = req.as_object_mut().unwrap();

    // METHOD, URL, HTTP VERSION
    // If we can't parse a method, URL or Http Version from the request, output in JSON as is
    if write_req_summary(writer, key, req_map).is_err() {
        wln!(writer, "undefined undefined HTTP/1.1");
        return;
    }

    // CONNECTING HOST FOR CLIENT REQUEST
    if key.eq("client_req") {
        if let Some(address) = req_map.remove("address") {
            w!(
                writer,
                "{:indent$}Connecting Host: {}",
                "",
                string_or_value!(address),
                indent = BASE_INDENT_SIZE
            );

            if let Some(port) = req_map.remove("port") {
                w!(writer, ":{}", string_or_value!(port));
            }

            wln!(writer);
        }
    }

    // HTTP HEADERS
    if let Some(headers) = req_map.remove("headers") {
        write_headers(writer, &headers);
    }

    // HTTP BODY
    if let Some(body) = req_map.remove("body") {
        if let Some(body_map) = body.as_object() {
            let pretty =
                ::serde_json::to_string_pretty(&body_map).unwrap_or("[malformed]".to_string());
            for line in pretty.lines() {
                wln!(writer, "{:indent$}{}", "", line, indent = BASE_INDENT_SIZE);
            }
        } else {
            let body_text = string_or_value!(body);
            wln!(
                writer,
                "{:indent$}{}",
                "",
                body_text,
                indent = BASE_INDENT_SIZE
            );
        }
    }

    // HTTP TRAILER HEADERS
    if let Some(trailers) = req_map.remove("trailers") {
        write_headers(writer, &trailers);
    }
}

/// Converts the passed JSON value to an unsigned integer converting a numeric string or a
/// JSON numeric type.
///
/// # Errors
///
/// If there are any problems converting the value to an unsigned 16 bit integer, then
/// `None` will be returned.
///
fn json_string_or_number_as_u16(val: &Value) -> Result<u16, ParseIntFromJsonError> {
    match val {
        Value::Number(number) => {
            if let Some(code) = number.as_u64() {
                if code > u64::from(std::u16::MAX) {
                    let err = BunyanLogParseError::new(format!(
                        "Number is greater than u16 bounds: {}",
                        code
                    ));
                    Err(ParseIntFromJsonError::Structural(err))
                } else {
                    Ok(code as u16)
                }
            } else {
                let err = BunyanLogParseError::new(format!(
                    "Number can't be converted to u64: {}",
                    string_or_value!(val)
                ));
                Err(ParseIntFromJsonError::Structural(err))
            }
        }
        Value::String(string) => {
            let code = string.parse::<u16>();

            match code {
                Ok(val) => Ok(val),
                Err(e) => Err(ParseIntFromJsonError::Numeric(e)),
            }
        }
        Value::Null => {
            let err = BunyanLogParseError::new("Integers can't be parsed from null nodes");
            Err(ParseIntFromJsonError::Structural(err))
        }
        Value::Object(_) => {
            let err = BunyanLogParseError::new("Integers can't be parsed from JSON objects");
            Err(ParseIntFromJsonError::Structural(err))
        }
        Value::Array(_) => {
            let err = BunyanLogParseError::new("Integers can't be parsed from JSON arrays");
            Err(ParseIntFromJsonError::Structural(err))
        }
        Value::Bool(_) => {
            let err = BunyanLogParseError::new("Integers can't be parsed from boolean values");
            Err(ParseIntFromJsonError::Structural(err))
        }
    }
}

/// Writes the HTTP response information logged for the line.
///
/// # Arguments
///
/// * `writer` - Write implementation to output data to
/// * `other` - Map containing all non-explicitly deserialized keys and values
///
fn write_res<W: Write>(writer: &mut W, key: &str, other: &mut Map<String, Value>) {
    /// Searches the passed map for the key `headers` and then `header` returning whichever
    /// is found first and is a valid string or JSON object. Otherwise, `None` is returned.
    fn find_headers(map: &mut Map<String, Value>) -> Option<Value> {
        if let Some(headers) = map.remove("headers") {
            if headers.is_string() || headers.is_object() {
                return Some(headers);
            }
        }

        if let Some(headers) = map.remove("header") {
            if headers.is_string() || headers.is_object() {
                return Some(headers);
            }
        }

        None
    }

    /// Converts the passed JSON value and writes it out as a HTTP status code.
    ///
    /// # Errors
    ///
    /// If we can't parse the status code, then we behave as if there was no status code logged.
    ///
    fn write_res_status_code<W: Write>(
        writer: &mut W,
        optional_code: Option<Value>,
        option_http_version: Option<&str>,
    ) {
        let numeric_status_code = if let Some(json_value) = optional_code {
            match json_string_or_number_as_u16(&json_value) {
                Err(_) => None,
                Ok(number) => Some(number),
            }
        } else {
            None
        };

        if let Some(code) = numeric_status_code {
            let http_version = option_http_version.unwrap_or(DEFAULT_HTTP_VERSION);
            let http_status = format!("HTTP/{}", http_version);

            w!(
                writer,
                "{:indent$}{}",
                "",
                http_status.cyan(),
                indent = BASE_INDENT_SIZE
            );

            let color = if code >= 100 && code <= 199 {
                "blue"
            } else if code >= 200 && code <= 299 {
                "green"
            } else if code >= 300 && code <= 399 {
                "magenta"
            } else if code >= 400 && code <= 499 {
                "yellow"
            } else if code >= 500 && code <= 599 {
                "red"
            } else {
                "white"
            };

            let status_code = StatusCode::from(code);
            let response_status = format!(" {} {}", code, status_code.reason_phrase());
            w!(writer, "{}", response_status.color(color));
            wln!(writer);
        }
    }

    let res_option = other.remove(key);

    // If there is no res key, then just exit right away because there is nothing to do
    if res_option.is_none() {
        return;
    }

    let mut res = res_option.unwrap();

    if !res.is_object() {
        return;
    }

    let res_map = res.as_object_mut().unwrap();

    // HEADERS
    if let Some(ref headers) = find_headers(res_map) {
        match headers {
            Value::String(headers_str) => {
                let http_version = if headers_str.starts_with("HTTP/") {
                    Some(&headers_str[5..8])
                } else {
                    None
                };

                write_res_status_code(writer, res_map.remove("statusCode"), http_version);

                let lines = headers_str.lines();

                for line in lines {
                    if line.is_empty() {
                        continue;
                    }
                    wln!(writer, "{:indent$}{}", "", line, indent = BASE_INDENT_SIZE);
                }
            }
            Value::Object(_) => {
                write_res_status_code(writer, res_map.remove("statusCode"), None);
                write_headers(writer, headers);
            }
            _ => (),
        }
    // Attempt to write out the status code line, even if we don't have headers
    } else {
        write_res_status_code(writer, res_map.remove("statusCode"), None);
    }

    // BODY
    if let Some(body_val) = res_map.remove("body") {
        let body = string_or_value!(body_val);

        if !body.is_empty() {
            wln!(writer);
            for line in body.lines() {
                wln!(writer, "{:indent$}{}", "", line, indent = BASE_INDENT_SIZE);
            }
        }
    }
}

/// Writes the contents of `headers` or `header` in the passed map.
///
/// # Arguments
///
/// * `writer` - Write implementation to output data to
/// * `caller_name` - text indicating if we have been invoked from a "req" or "client_req" code path
/// * `headers` - Mutable map containing header(s) keys. Keys will be removed as processed.
///
fn write_headers<W: Write>(writer: &mut W, headers: &Value) {
    match headers {
        Value::String(headers_string) => {
            for line in headers_string.lines() {
                if line.trim().is_empty() {
                    continue;
                }

                wln!(writer, "{:indent$}{}", "", line, indent = BASE_INDENT_SIZE);
            }
        }
        Value::Object(headers_map) => {
            for (k, v) in headers_map.iter() {
                w!(writer, "{:indent$}{}:", "", k, indent = BASE_INDENT_SIZE);

                let mut is_first = true;

                for line in string_or_value!(v).lines() {
                    if is_first {
                        wln!(writer, " {}", line);
                        is_first = false;
                    } else {
                        wln!(writer, "{:indent$}{}", "", line, indent = BASE_INDENT_SIZE);
                    }
                }
            }
        }
        _ => (),
    }
}

/// Writes the error information for the log line.
///
/// # Arguments
///
/// * `writer` - Write implementation to output data to
/// * `other` - Map containing all non-explicitly deserialized keys and values
///
fn write_err<W: Write>(writer: &mut W, other: &mut Map<String, Value>) {
    let err_option = other.remove("err");

    if err_option.is_none() {
        return;
    }

    let mut err = err_option.unwrap();

    if !err.is_object() {
        return;
    }

    let err_map = err.as_object_mut().unwrap();

    if let Some(ref stack_val) = err_map.remove("stack") {
        match stack_val {
            Value::String(stack_str) => {
                for line in stack_str.lines() {
                    wln!(writer, "{:indent$}{}", "", line, indent = BASE_INDENT_SIZE);
                }
            }
            Value::Array(stack_array) => {
                for line in stack_array.iter() {
                    wln!(
                        writer,
                        "{:indent$}{}",
                        "",
                        string_or_value!(line),
                        indent = BASE_INDENT_SIZE
                    );
                }
            }
            _ => {
                let pretty =
                    ::serde_json::to_string_pretty(&stack_val).unwrap_or("[malformed]".to_string());
                for line in pretty.lines() {
                    wln!(writer, "{:indent$}{}", "", line, indent = BASE_INDENT_SIZE);
                }
            }
        }
    }
}

/// Writes the accumulated "details parameters" that do not properly fit in the "extra parameters"
/// section of the output.
///
/// # Arguments
///
/// * `writer` - Write implementation to output data to
/// * `details` - Vector containing the parameters to write out each in their own section
///
fn write_details<W: Write>(divider_writer: &mut DividerWriter<W>, details: Vec<String>) {
    for item in details {
        for line in item.lines() {
            wln!(
                divider_writer,
                "{:indent$}{}",
                "",
                line,
                indent = BASE_INDENT_SIZE
            );
        }

        if divider_writer.has_been_written {
            divider_writer.mark_divider_as_unwritten();
        }
    }
}

/// Validates that the passed `BunyanLine` is of the correct structure where it can be parsed
/// without problems.
///
/// # Arguments
///
/// * `line` - log line to validate
///
/// # Errors
///
/// This function will return None if no errors have been encountered. In the case of parsing logic
/// errors where the JSON data is not in the expected format, it will return a
/// `Option<BunyanLogParseError>`.
fn validate_log_data_structure(line: &BunyanLine) -> Option<BunyanLogParseError> {
    fn find_headers(map: &Map<String, Value>) -> Option<&Value> {
        if let Some(headers) = map.get("headers") {
            if headers.is_string() || headers.is_object() {
                return Some(headers);
            }
        }

        if let Some(headers) = map.get("header") {
            if headers.is_string() || headers.is_object() {
                return Some(headers);
            }
        }

        None
    }

    // Validate src
    if let Some(ref src) = line.other.get("src") {
        match src {
            Value::Object(src_map) => {
                if let Some(ref file) = src_map.get("file") {
                    if !file.is_string() {
                        return Some(BunyanLogParseError::new("[src.file] must be a string"));
                    }
                }
                if let Some(ref line) = src_map.get("line") {
                    if !(line.is_string() || line.is_number()) {
                        return Some(BunyanLogParseError::new(
                            "[src.line] must be a number or string",
                        ));
                    }
                }
                if let Some(ref func) = src_map.get("func") {
                    if !func.is_string() {
                        return Some(BunyanLogParseError::new("[src.func] must be a string"));
                    }
                }
            }
            Value::String(_) => (),
            _ => {
                return Some(BunyanLogParseError::new(
                    "[src] value must be a JSON object or string",
                ));
            }
        }
    }

    // Validate req_id
    if let Some(ref req_id) = line.other.get("req_id") {
        if !(req_id.is_string() || req_id.is_number() || req_id.is_null()) {
            return Some(BunyanLogParseError::new(
                "[req_id] must be a string or number",
            ));
        }
    }

    // Validate req
    if let Some(ref client_req) = line.other.get("req") {
        if let Some(body) = client_req.get("body") {
            if body.is_array() {
                return Some(BunyanLogParseError::new(
                    "[req.body] value must be not be an array",
                ));
            }
        }
    }

    // Validate client_req
    if let Some(ref client_req) = line.other.get("client_req") {
        if let Some(ref address) = client_req.get("address") {
            if !(address.is_string() || address.is_number() || address.is_null()) {
                return Some(BunyanLogParseError::new(
                    "[client_req.address] value must be a string",
                ));
            }
        }
        if let Some(ref port) = client_req.get("port") {
            if !(port.is_string() || port.is_number() || port.is_null()) {
                return Some(BunyanLogParseError::new(
                    "[client_req.port] value must be a string or number",
                ));
            }
        }

        if let Some(body) = client_req.get("body") {
            if body.is_array() {
                return Some(BunyanLogParseError::new(
                    "[client_req.body] value must be not be an array",
                ));
            }
        }
    }

    // Validate res
    if let Some(ref res) = line.other.get("res") {
        if let Some(ref res_map) = res.as_object() {
            if let Some(headers) = find_headers(res_map) {
                if !(headers.is_object() || headers.is_string()) {
                    return Some(BunyanLogParseError::new(
                        "[res.header(s)] must be a JSON object or string",
                    ));
                }
            }
        }

        if let Some(ref status_code) = res.get("statusCode") {
            if let Err(e) = json_string_or_number_as_u16(status_code) {
                let msg = format!("Invalid status code on res: {}", e);
                return Some(BunyanLogParseError::new(msg));
            }
        }
    }

    // Validate client_res
    if let Some(ref res) = line.other.get("client_res") {
        if let Some(ref res_map) = res.as_object() {
            if let Some(headers) = find_headers(res_map) {
                if !(headers.is_object() || headers.is_string()) {
                    return Some(BunyanLogParseError::new(
                        "[client_res.header(s)] must be a JSON object or string",
                    ));
                }
            }
            if let Some(body) = res.get("body") {
                if body.is_array() {
                    return Some(BunyanLogParseError::new(
                        "[client_res.body] value must be not be an array",
                    ));
                }
                if body.is_object() {
                    return Some(BunyanLogParseError::new(
                        "[client_res.body] value must be not be a JSON object",
                    ));
                }
            }
        }

        if let Some(ref status_code) = res.get("statusCode") {
            if let Err(e) = json_string_or_number_as_u16(status_code) {
                let msg = format!("Invalid status code on client_res: {}", e);
                return Some(BunyanLogParseError::new(msg));
            }
        }
    }

    None
}

impl Logger for BunyanLine {
    fn write_long_format<W: Write>(
        &self,
        writer: &mut W,
        _output_config: &LoggerOutputConfig,
    ) -> ParseResult {
        if let Some(err) = validate_log_data_structure(&self) {
            return Err(err);
        }

        let log_level: LogLevel = self.level.into();

        // Write the [time]
        w!(writer, "{}{}{}", "[".blue(), self.time.bright_white(), "]".blue());

        // write the log [level] and app [name]
        w!(writer, " {}: {}/", log_level, self.name);

        // If present, write the [component]
        if let Some(ref component) = self.component {
            w!(writer, "{}/", component);
        }

        // Write the [pid] and [hostname]
        w!(writer, "{} on {}", self.pid, self.hostname);

        let other = &mut self.other.clone();

        // If present, write the source line reference [src]
        write_src(writer, other);

        let mut details: Vec<String> = Vec::new();

        // If our log message [msg] contains a line break, we display it in the details section
        if self.msg.contains('\n') {
            let indented_msg = format!("{:indent$}{}", "", self.msg, indent = BASE_INDENT_SIZE);
            details.push(indented_msg)
        // Write the log message [msg] as is because there is no line break
        } else if !self.msg.is_empty() {
            w!(writer, ": {}", self.msg.cyan());
        } else {
            w!(writer, ":");
        }

        write_all_extra_params(writer, other, &mut details);

        // Write line feed finishing the first line
        wln!(writer);

        let wrapped_writer = &mut DividerWriter::new(writer, true);

        // If present, write the request [req]
        write_req(wrapped_writer, "req", other);

        if wrapped_writer.has_been_written {
            wrapped_writer.mark_divider_as_unwritten();
        }

        // If present, write the client request [client_req]
        write_req(wrapped_writer, "client_req", other);

        if wrapped_writer.has_been_written {
            wrapped_writer.mark_divider_as_unwritten();
        }

        // If present, write the response [res]
        write_res(wrapped_writer, "res", other);

        if wrapped_writer.has_been_written {
            wrapped_writer.mark_divider_as_unwritten();
        }

        // If present, write the response [client_res]
        write_res(wrapped_writer, "client_res", other);

        if wrapped_writer.has_been_written {
            wrapped_writer.mark_divider_as_unwritten();
        }

        // If present, write the error information [err]
        write_err(wrapped_writer, other);

        if wrapped_writer.has_been_written {
            wrapped_writer.mark_divider_as_unwritten();
        }

        // Write out all of the values stored in the details vector
        write_details(wrapped_writer, details);

        Ok(())
    }
}
