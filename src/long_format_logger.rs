use std::io::Write;

use chrono::{Local, SecondsFormat};

use colored::*;

use crate::divider_writer::DividerWriter;
use crate::errors::ParseResult;
use crate::format_logger_helpers::*;
use crate::{BunyanLine, LogLevel, Logger, LoggerOutputConfig, BASE_INDENT_SIZE};

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
        let time = if _output_config.display_local_time {
            self.time
                .with_timezone(&Local)
                .to_rfc3339_opts(SecondsFormat::Millis, true)
        } else {
            self.time.to_rfc3339_opts(SecondsFormat::Millis, true)
        };

        w!(
            writer,
            "{}{}{}",
            "[".blue(),
            time.bright_white(),
            "]".blue()
        );

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
