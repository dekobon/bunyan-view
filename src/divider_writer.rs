use std::io::{Write, Error};
use std::fmt::Arguments;
use crate::BASE_INDENT_SIZE;

/// Textual divider between log sections
const DIVIDER: &str = "--\n";

pub struct DividerWriter<'a, W: Write> {
    inner: &'a mut W,
    pub divider_written: bool,
    pub has_been_written: bool
}

impl<'a, W: Write> DividerWriter<'a, W> {
    pub fn new(inner: &'a mut W, divider_written: bool) -> DividerWriter<W> {
        DividerWriter {
            inner: inner,
            divider_written: divider_written.clone(),
            has_been_written: false
        }
    }

    pub fn mark_divider_as_unwritten(&mut self) {
        self.divider_written = false;
    }

    fn write_divider(&mut self) {
        // "{:indent$}", "", indent = BASE_INDENT_SIZE
        if let Err(e) = self.inner.write_fmt(format_args!("{:indent$}{}", "", DIVIDER,
                                                          indent = BASE_INDENT_SIZE)) {
            panic!(e);
        }

        self.divider_written = true;
    }
}

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