extern crate bunyan_view;
extern crate bytes;
#[macro_use] extern crate pretty_assertions;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use bytes::BufMut;
use bunyan_view::LoggerOutputConfig;

fn assert_equals_to_file(filename: &str) {
    let format = bunyan_view::LogFormat::Long;
    let mut writer = vec![].writer();
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    let expected_filename = &[filename, ".expected"].concat();
    let mut expected_file = File::open(expected_filename).expect("file not found");

    let mut expected = String::new();
    expected_file.read_to_string(&mut expected)
        .expect(&["There was a problem opening the expectation file: ", expected_filename].concat());

    let output_config = LoggerOutputConfig {
        indent: 4,
        is_debug: false,
        is_strict: false,
        level: None
    };

    bunyan_view::write_bunyan_output(&mut writer, Box::new(reader), &format,
                                     output_config);
    let actual_bytes: Vec<u8> = writer.into_inner();
    let actual = std::str::from_utf8(&actual_bytes).expect("Couldn't convert bytes");

    let expected_lines = expected.lines();
    let actual_lines = actual.lines();
    let zipped_lines = expected_lines.zip(actual_lines);

    let mut pos: u16 = 0;

    // Compare line by line in order to get better visibility if there is a difference
    for (expected_line, actual_line) in zipped_lines {
        pos += 1;
        assert_eq!(format!("{}: {}", pos, actual_line),
                   format!("{}: {}", pos, expected_line));
    }

    // Lastly, compare the entire output to make sure we are completely meeting the expectation
    assert_eq!(actual, expected);
}

/* ============================================================================================== *\
 * Test corpus files from node bunyan
 * ============================================================================================== */

#[test]
fn long_format_simple() {
    assert_equals_to_file("tests/corpus/simple.log");
}

#[test]
fn long_format_extrafield() {
    assert_equals_to_file("tests/corpus/extrafield.log");
}

#[test]
fn long_format_bogus() {
    assert_equals_to_file("tests/corpus/bogus.log");
}

#[test]
fn long_format_withreq() {
    assert_equals_to_file("tests/corpus/withreq.log");
}

#[test]
fn long_format_all() {
    assert_equals_to_file("tests/corpus/all.log");
}

#[test]
fn long_client_req_with_address() {
    assert_equals_to_file("tests/corpus/client-req-with-address.log");
}

#[test]
fn long_client_reqres() {
    assert_equals_to_file("tests/corpus/clientreqres.log");
}

#[test]
fn long_content_length_zero_res() {
    assert_equals_to_file("tests/corpus/content-length-0-res.log");
}

#[test]
fn long_log_1() {
    assert_equals_to_file("tests/corpus/log1.log");
}

#[test]
fn long_log_2() {
    assert_equals_to_file("tests/corpus/log2.log");
}

#[test]
fn long_non_object_res() {
    assert_equals_to_file("tests/corpus/non-object-res.log");
}

#[test]
fn long_res_header() {
    assert_equals_to_file("tests/corpus/res-header.log");
}

#[test]
fn long_res_without_header() {
    assert_equals_to_file("tests/corpus/res-without-header.log");
}

#[test]
fn long_old_crashers_139() {
    assert_equals_to_file("tests/corpus/old-crashers/139.log");
}

#[test]
fn long_old_crashers_144() {
    assert_equals_to_file("tests/corpus/old-crashers/144.log");
}

#[test]
fn long_old_crashers_233() {
    assert_equals_to_file("tests/corpus/old-crashers/233.log");
}

#[test]
fn long_old_crashers_242() {
    assert_equals_to_file("tests/corpus/old-crashers/242.log");
}

#[test]
fn long_old_crashers_244() {
    assert_equals_to_file("tests/corpus/old-crashers/244.log");
}

/* ============================================================================================== *\
 * Test corpus files created for rust bunyan view
 * ============================================================================================== */

#[test]
fn long_error_with_stack() {
    assert_equals_to_file("tests/corpus/error-with-stack.log");
}

#[test]
fn long_req_with_newlines() {
    assert_equals_to_file("tests/corpus/req-with-newlines.log");
}

#[test]
fn long_req_with_trailers() {
    assert_equals_to_file("tests/corpus/req-with-trailers.log");
}

#[test]
fn long_res_with_empty_object() {
    assert_equals_to_file("tests/corpus/res-with-empty-object.log");
}

#[test]
fn long_with_numeric_req_id() {
    assert_equals_to_file("tests/corpus/with-numeric-req-id.log");
}

#[test]
fn long_with_weird_extra_params() {
    assert_equals_to_file("tests/corpus/with-weird-extra-params.log");
}