macro_rules! w {
    ($dst:expr, $($arg:tt)*) => {{
        let result = $dst.write_fmt(format_args!($($arg)*));

        match result {
            Ok(_) => {},
            Err(_) => {
                // Exit without message because this is likely a SIGPIPE
                ::std::process::exit(1);
            }
        }
    }}
}

macro_rules! wln {
    ($dst:expr) => (
        w!($dst, "\n")
    );
    ($dst:expr,) => (
        w!($dst)
    );
    ($dst:expr, $($arg:tt)*) => {{
        let result = $dst.write_fmt(format_args!($($arg)*));
        match result {
            Ok(_) => {},
            Err(_) => {
                // Exit without message because this is likely a SIGPIPE
                ::std::process::exit(1);
            }
        };
        w!($dst, "\n");
    }};
}

macro_rules! string_or_value {
    ($val:expr) => {
        if $val.is_string() {
            $val.as_str().unwrap_or("undefined").to_string()
        } else if $val.is_null() {
            "null".to_string()
        } else {
            $val.to_string()
        }
    };
}
