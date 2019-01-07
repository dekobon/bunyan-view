use crate::BASE_INDENT_SIZE;
use std::fmt::Arguments;
use std::io::{Error, Write};

/// Textual divider between log sections
const DIVIDER: &str = "--\n";

/// Struct whose purpose is to wrap any instance that implements the `Write` trait in order to
/// inject periodical dividers into the wrapped `Write` implementation.
///
pub struct DividerWriter<'a, W: Write> {
    inner: &'a mut W,
    pub divider_written: bool,
    pub has_been_written: bool,
}

impl<'a, W: Write> DividerWriter<'a, W> {
    pub fn new(inner: &'a mut W, divider_written: bool) -> DividerWriter<W> {
        DividerWriter {
            inner,
            divider_written,
            has_been_written: false,
        }
    }

    /// Marks the instance as having not yet written a divider.
    ///
    pub fn mark_divider_as_unwritten(&mut self) {
        self.divider_written = false;
    }

    /// Writes the divider to the wrapped `Write` instance and marks the divider as written.
    ///
    fn write_divider(&mut self) {
        // "{:indent$}", "", indent = BASE_INDENT_SIZE
        if let Err(e) = self.inner.write_fmt(format_args!(
            "{:indent$}{}",
            "",
            DIVIDER,
            indent = BASE_INDENT_SIZE
        )) {
            panic!(e);
        }

        self.divider_written = true;
    }
}

/// Wrap all of the behavior of the inner `Write` instance.
///
impl<'a, W: Write> Write for DividerWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        if !&self.has_been_written {
            self.has_been_written = true;
        }
        if !&self.divider_written {
            self.write_divider();
        }

        self.inner.write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.inner.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        if !&self.has_been_written {
            self.has_been_written = true;
        }
        if !&self.divider_written {
            self.write_divider();
        }

        self.inner.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments) -> Result<(), Error> {
        if !&self.has_been_written {
            self.has_been_written = true;
        }
        if !&self.divider_written {
            self.write_divider();
        }

        self.inner.write_fmt(fmt)
    }
}
