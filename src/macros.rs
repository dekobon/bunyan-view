macro_rules! w {
    ($dst:expr, $($arg:tt)*) => {{
        let result = $dst.write_fmt(format_args!($($arg)*));

        match result {
            Ok(_) => {},
            Err(e) => {
                panic!(e);
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
            Err(e) => {
                panic!(e);
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

macro_rules! get_or_default{
    ($map:expr, $key:expr, $default:expr) => {
        if let Some(ref val) = $map.get($key) {
            if val.is_string() {
                val.as_str().unwrap_or($default).to_string()
            } else {
                val.to_string()
            }
        } else {
            $default.to_string()
        }
    }
}