use std::convert::TryInto;
use std::io::Write;

use serde_json::map::Map;
use serde_json::Value;

use colored::*;

/// Default indent size in spaces
const BASE_INDENT_SIZE: usize = 2;

/// Writes the root portion of each log entry starting on a new line.
///
/// Arguments
///
/// * `writer` - Write implementation to output data to
/// * `map` - Mutable map containing JSON data.
///
pub fn write_inspect_line<W: Write>(writer: &mut W, map: Map<String, Value>)
where
    W: Write,
{
    w!(writer, "{}{{\n", "".bright_white());
    let leading_spaces = " ".repeat(BASE_INDENT_SIZE);
    let mut itr = map.into_iter().peekable();

    while let Some(next) = itr.next() {
        let (k, v) = next;

        w!(writer, "{}{}: ", leading_spaces, k.bright_white());

        write_value(writer, v, BASE_INDENT_SIZE * 2);

        if itr.peek().is_some() {
            w!(writer, "{}\n", ",".bright_white());
        }
    }

    wln!(writer, "\n}}");
}

/// Writes the value of a given JSON entry. This could be a string, number, array or object.
///
/// Arguments
///
/// * `writer` - Write implementation to output data to
/// * `value` - Entry to write
/// * `indent` - number of spaces to indent
///
fn write_value<W: Write>(writer: &mut W, value: Value, indent: usize)
where
    W: Write,
{
    match value {
        Value::String(string) => {
            let line = if string.len() > 80 {
                format_long_line(string, indent)
            } else {
                escape(string).replace("\n", "\\n")
            };

            w!(writer, "{}{}{}", "'".green(), line.green(), "'".green());
        }
        Value::Number(number) => w!(writer, "{}", number.to_string().yellow()),
        Value::Array(array) => write_array(writer, array, indent),
        Value::Object(obj) => write_object(writer, obj, indent),
        _ => w!(writer, "{}{}", "".green(), value),
    }
}

/// Writes a JSON array.
///
/// Arguments
///
/// * `writer` - Write implementation to output data to
/// * `array` - array to write
/// * `indent` - number of spaces to indent
///
fn write_array<W: Write>(writer: &mut W, array: Vec<Value>, indent: usize)
where
    W: Write,
{
    let mut itr = array.into_iter().peekable();

    w!(writer, "{}", "[".bright_white());

    if itr.peek().is_some() {
        w!(writer, " ");
    }

    while let Some(next) = itr.next() {
        write_value(writer, next, indent + BASE_INDENT_SIZE);

        if itr.peek().is_some() {
            w!(writer, "{}", ", ");
        } else {
            w!(writer, "{}", " ");
        }
    }

    w!(writer, "{}", "]".bright_white());
}

/// Writes a JSON object.
///
/// Arguments
///
/// * `writer` - Write implementation to output data to
/// * `obj` - object to write
/// * `indent` - number of spaces to indent
///
fn write_object<W: Write>(writer: &mut W, obj: Map<String, Value>, indent: usize)
where
    W: Write,
{
    w!(writer, "{}", "{".bright_white());

    let mut itr = obj.into_iter().peekable();
    // Empty objects should just return {}
    if itr.peek().is_none() {
        w!(writer, "{}", "}".bright_white());
        return;
    }

    if itr.peek().is_some() {
        w!(writer, "\n");
    }

    while let Some(next) = itr.next() {
        let (k, v) = next;

        w!(writer, "{}{}: ", " ".repeat(indent), k.bright_white());
        write_value(writer, v, indent + BASE_INDENT_SIZE);

        if itr.peek().is_some() {
            w!(writer, "{}\n", ",".bright_white());
        }
    }

    let trailing_indent: usize = isize::abs((indent - BASE_INDENT_SIZE) as isize)
        .try_into()
        .unwrap();
    let trailing_spaces = " ".repeat(trailing_indent);

    w!(writer, "\n{}{}", trailing_spaces, "}".bright_white());
}

/// Formats a long line into a Javascript style string with single quotes, pluses,
/// and a new line.
///
/// Arguments
///
/// * `string` - String to format
/// * `indent` - number of spaces to indent
///
fn format_long_line(string: String, indent: usize) -> String {
    // New lines are replaced with the \n in the JS inspect format
    let newline = format!("\\n' +\n{}'", " ".repeat(indent));
    let ends_with_newline = string.ends_with("\n");
    let escaped = escape(string);
    let trimmed = if ends_with_newline {
        escaped.trim_end().to_string()
    } else {
        escaped
    };
    let replaced = trimmed.replace("\n", newline.as_str());
    let line = if ends_with_newline {
        format!("{}{}", replaced, "\\n")
    } else {
        replaced
    };

    return line;
}

/// Escapes a string with slash style escapes.
/// Note: this function does not escape \n because that is done in separate logic
/// in order to accommodate Javascript style long strings.  
///
/// Arguments
///
/// * `string` - String to escape
///
fn escape(src: String) -> String {
    let mut escaped = String::with_capacity(src.len());
    for c in src.chars() {
        match c {
            '\x08' => escaped += "\\b",
            '\x0c' => escaped += "\\f",
            '\r' => escaped += "\\r",
            '\t' => escaped += "\\t",
            _ => (escaped.push(c)),
        }
    }
    escaped
}
