extern crate bunyan_view;
extern crate bytes;
#[macro_use]
extern crate pretty_assertions;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use bunyan_view::{LogFormat, LoggerOutputConfig};
use bytes::BufMut;

fn assert_equals_to_file(filename: &str, expected_filename: &str, format: LogFormat) {
    let mut writer = vec![].writer();
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let mut expected_file = File::open(expected_filename).expect("file not found");

    let mut expected = String::new();
    let msg = &[
        "There was a problem opening the expectation file: ",
        expected_filename,
    ]
    .concat();
    expected_file.read_to_string(&mut expected).expect(msg);

    let output_config = LoggerOutputConfig {
        indent: 4,
        is_debug: false,
        is_strict: false,
        level: None,
        display_local_time: false,
        format,
    };

    bunyan_view::write_bunyan_output(&mut writer, reader, &output_config);
    let actual_bytes: Vec<u8> = writer.into_inner();
    let actual = std::str::from_utf8(&actual_bytes).expect("Couldn't convert bytes");

    let expected_lines = expected.lines();
    let actual_lines = actual.lines();
    let zipped_lines = expected_lines.zip(actual_lines);

    let mut pos: u16 = 0;

    // Compare line by line in order to get better visibility if there is a difference
    for (expected_line, actual_line) in zipped_lines {
        pos += 1;
        assert_eq!(
            format!("{}: {}", pos, actual_line),
            format!("{}: {}", pos, expected_line)
        );
    }

    // Lastly, compare the entire output to make sure we are completely meeting the expectation
    assert_eq!(actual, expected);
}

// LONG FORMAT

/* ============================================================================================== *\
 * Test corpus files from node bunyan
 * ============================================================================================== */

#[test]
fn long_format_simple() {
    assert_equals_to_file(
        "tests/corpus/simple.log",
        "tests/expectations/long/simple.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_format_extrafield() {
    assert_equals_to_file(
        "tests/corpus/extrafield.log",
        "tests/expectations/long/extrafield.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_format_bogus() {
    assert_equals_to_file(
        "tests/corpus/bogus.log",
        "tests/expectations/long/bogus.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_format_withreq() {
    assert_equals_to_file(
        "tests/corpus/withreq.log",
        "tests/expectations/long/withreq.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_format_all() {
    assert_equals_to_file(
        "tests/corpus/all.log",
        "tests/expectations/long/all.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_client_req_with_address() {
    assert_equals_to_file(
        "tests/corpus/client-req-with-address.log",
        "tests/expectations/long/client-req-with-address.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_client_reqres() {
    assert_equals_to_file(
        "tests/corpus/clientreqres.log",
        "tests/expectations/long/clientreqres.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_content_length_zero_res() {
    assert_equals_to_file(
        "tests/corpus/content-length-0-res.log",
        "tests/expectations/long/content-length-0-res.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_log_1() {
    assert_equals_to_file(
        "tests/corpus/log1.log",
        "tests/expectations/long/log1.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_log_2() {
    assert_equals_to_file(
        "tests/corpus/log2.log",
        "tests/expectations/long/log2.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_non_object_res() {
    assert_equals_to_file(
        "tests/corpus/non-object-res.log",
        "tests/expectations/long/non-object-res.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_res_header() {
    assert_equals_to_file(
        "tests/corpus/res-header.log",
        "tests/expectations/long/res-header.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_res_without_header() {
    assert_equals_to_file(
        "tests/corpus/res-without-header.log",
        "tests/expectations/long/res-without-header.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_old_crashers_139() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/139.log",
        "tests/expectations/long/old-crashers/139.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_old_crashers_144() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/144.log",
        "tests/expectations/long/old-crashers/144.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_old_crashers_233() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/233.log",
        "tests/expectations/long/old-crashers/233.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_old_crashers_242() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/242.log",
        "tests/expectations/long/old-crashers/242.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_old_crashers_244() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/244.log",
        "tests/expectations/long/old-crashers/244.log.expected",
        LogFormat::Long,
    );
}

/* ============================================================================================== *\
 * Test corpus files created for rust bunyan view
 * ============================================================================================== */

#[test]
fn long_error_with_stack() {
    assert_equals_to_file(
        "tests/corpus/error-with-stack.log",
        "tests/expectations/long/error-with-stack.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_req_with_newlines() {
    assert_equals_to_file(
        "tests/corpus/req-with-newlines.log",
        "tests/expectations/long/req-with-newlines.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_req_with_trailers() {
    assert_equals_to_file(
        "tests/corpus/req-with-trailers.log",
        "tests/expectations/long/req-with-trailers.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_res_with_empty_object() {
    assert_equals_to_file(
        "tests/corpus/res-with-empty-object.log",
        "tests/expectations/long/res-with-empty-object.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_with_numeric_req_id() {
    assert_equals_to_file(
        "tests/corpus/with-numeric-req-id.log",
        "tests/expectations/long/with-numeric-req-id.log.expected",
        LogFormat::Long,
    );
}

#[test]
fn long_with_weird_extra_params() {
    assert_equals_to_file(
        "tests/corpus/with-weird-extra-params.log",
        "tests/expectations/long/with-weird-extra-params.log.expected",
        LogFormat::Long,
    );
}

// SHORT FORMAT

/* ============================================================================================== *\
 * Test corpus files from node bunyan
 * ============================================================================================== */

#[test]
fn short_format_simple() {
    assert_equals_to_file(
        "tests/corpus/simple.log",
        "tests/expectations/short/simple.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_format_extrafield() {
    assert_equals_to_file(
        "tests/corpus/extrafield.log",
        "tests/expectations/short/extrafield.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_format_bogus() {
    assert_equals_to_file(
        "tests/corpus/bogus.log",
        "tests/expectations/short/bogus.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_format_withreq() {
    assert_equals_to_file(
        "tests/corpus/withreq.log",
        "tests/expectations/short/withreq.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_format_all() {
    assert_equals_to_file(
        "tests/corpus/all.log",
        "tests/expectations/short/all.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_client_req_with_address() {
    assert_equals_to_file(
        "tests/corpus/client-req-with-address.log",
        "tests/expectations/short/client-req-with-address.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_client_reqres() {
    assert_equals_to_file(
        "tests/corpus/clientreqres.log",
        "tests/expectations/short/clientreqres.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_content_length_zero_res() {
    assert_equals_to_file(
        "tests/corpus/content-length-0-res.log",
        "tests/expectations/short/content-length-0-res.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_log_1() {
    assert_equals_to_file(
        "tests/corpus/log1.log",
        "tests/expectations/short/log1.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_log_2() {
    assert_equals_to_file(
        "tests/corpus/log2.log",
        "tests/expectations/short/log2.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_non_object_res() {
    assert_equals_to_file(
        "tests/corpus/non-object-res.log",
        "tests/expectations/short/non-object-res.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_res_header() {
    assert_equals_to_file(
        "tests/corpus/res-header.log",
        "tests/expectations/short/res-header.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_res_without_header() {
    assert_equals_to_file(
        "tests/corpus/res-without-header.log",
        "tests/expectations/short/res-without-header.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_old_crashers_139() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/139.log",
        "tests/expectations/short/old-crashers/139.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_old_crashers_144() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/144.log",
        "tests/expectations/short/old-crashers/144.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_old_crashers_233() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/233.log",
        "tests/expectations/short/old-crashers/233.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_old_crashers_242() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/242.log",
        "tests/expectations/short/old-crashers/242.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_old_crashers_244() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/244.log",
        "tests/expectations/short/old-crashers/244.log.expected",
        LogFormat::Short,
    );
}

/* ============================================================================================== *\
 * Test corpus files created for rust bunyan view
 * ============================================================================================== */

#[test]
fn short_error_with_stack() {
    assert_equals_to_file(
        "tests/corpus/error-with-stack.log",
        "tests/expectations/short/error-with-stack.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_req_with_newlines() {
    assert_equals_to_file(
        "tests/corpus/req-with-newlines.log",
        "tests/expectations/short/req-with-newlines.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_req_with_trailers() {
    assert_equals_to_file(
        "tests/corpus/req-with-trailers.log",
        "tests/expectations/short/req-with-trailers.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_res_with_empty_object() {
    assert_equals_to_file(
        "tests/corpus/res-with-empty-object.log",
        "tests/expectations/short/res-with-empty-object.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_with_numeric_req_id() {
    assert_equals_to_file(
        "tests/corpus/with-numeric-req-id.log",
        "tests/expectations/short/with-numeric-req-id.log.expected",
        LogFormat::Short,
    );
}

#[test]
fn short_with_weird_extra_params() {
    assert_equals_to_file(
        "tests/corpus/with-weird-extra-params.log",
        "tests/expectations/short/with-weird-extra-params.log.expected",
        LogFormat::Short,
    );
}

// SIMPLE FORMAT

/* ============================================================================================== *\
 * Test corpus files from node bunyan
 * ============================================================================================== */

#[test]
fn simple_format_simple() {
    assert_equals_to_file(
        "tests/corpus/simple.log",
        "tests/expectations/simple/simple.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_format_extrafield() {
    assert_equals_to_file(
        "tests/corpus/extrafield.log",
        "tests/expectations/simple/extrafield.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_format_bogus() {
    assert_equals_to_file(
        "tests/corpus/bogus.log",
        "tests/expectations/simple/bogus.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_format_withreq() {
    assert_equals_to_file(
        "tests/corpus/withreq.log",
        "tests/expectations/simple/withreq.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_format_all() {
    assert_equals_to_file(
        "tests/corpus/all.log",
        "tests/expectations/simple/all.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_client_req_with_address() {
    assert_equals_to_file(
        "tests/corpus/client-req-with-address.log",
        "tests/expectations/simple/client-req-with-address.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_client_reqres() {
    assert_equals_to_file(
        "tests/corpus/clientreqres.log",
        "tests/expectations/simple/clientreqres.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_content_length_zero_res() {
    assert_equals_to_file(
        "tests/corpus/content-length-0-res.log",
        "tests/expectations/simple/content-length-0-res.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_log_1() {
    assert_equals_to_file(
        "tests/corpus/log1.log",
        "tests/expectations/simple/log1.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_log_2() {
    assert_equals_to_file(
        "tests/corpus/log2.log",
        "tests/expectations/simple/log2.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_non_object_res() {
    assert_equals_to_file(
        "tests/corpus/non-object-res.log",
        "tests/expectations/simple/non-object-res.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_res_header() {
    assert_equals_to_file(
        "tests/corpus/res-header.log",
        "tests/expectations/simple/res-header.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_res_without_header() {
    assert_equals_to_file(
        "tests/corpus/res-without-header.log",
        "tests/expectations/simple/res-without-header.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_old_crashers_139() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/139.log",
        "tests/expectations/simple/old-crashers/139.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_old_crashers_144() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/144.log",
        "tests/expectations/simple/old-crashers/144.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_old_crashers_233() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/233.log",
        "tests/expectations/simple/old-crashers/233.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_old_crashers_242() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/242.log",
        "tests/expectations/simple/old-crashers/242.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_old_crashers_244() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/244.log",
        "tests/expectations/simple/old-crashers/244.log.expected",
        LogFormat::Simple,
    );
}

/* ============================================================================================== *\
 * Test corpus files created for rust bunyan view
 * ============================================================================================== */

#[test]
fn simple_error_with_stack() {
    assert_equals_to_file(
        "tests/corpus/error-with-stack.log",
        "tests/expectations/simple/error-with-stack.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_req_with_newlines() {
    assert_equals_to_file(
        "tests/corpus/req-with-newlines.log",
        "tests/expectations/simple/req-with-newlines.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_req_with_trailers() {
    assert_equals_to_file(
        "tests/corpus/req-with-trailers.log",
        "tests/expectations/simple/req-with-trailers.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_res_with_empty_object() {
    assert_equals_to_file(
        "tests/corpus/res-with-empty-object.log",
        "tests/expectations/simple/res-with-empty-object.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_with_numeric_req_id() {
    assert_equals_to_file(
        "tests/corpus/with-numeric-req-id.log",
        "tests/expectations/simple/with-numeric-req-id.log.expected",
        LogFormat::Simple,
    );
}

#[test]
fn simple_with_weird_extra_params() {
    assert_equals_to_file(
        "tests/corpus/with-weird-extra-params.log",
        "tests/expectations/simple/with-weird-extra-params.log.expected",
        LogFormat::Simple,
    );
}

// INSPECT FORMAT

/* ============================================================================================== *\
 * Test corpus files from node bunyan
 * ============================================================================================== */

#[test]
fn inspect_format_simple() {
    assert_equals_to_file(
        "tests/corpus/simple.log",
        "tests/expectations/inspect/simple.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_format_extrafield() {
    assert_equals_to_file(
        "tests/corpus/extrafield.log",
        "tests/expectations/inspect/extrafield.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_format_bogus() {
    assert_equals_to_file(
        "tests/corpus/bogus.log",
        "tests/expectations/inspect/bogus.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_format_withreq() {
    assert_equals_to_file(
        "tests/corpus/withreq.log",
        "tests/expectations/inspect/withreq.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_format_all() {
    assert_equals_to_file(
        "tests/corpus/all.log",
        "tests/expectations/inspect/all.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_client_req_with_address() {
    assert_equals_to_file(
        "tests/corpus/client-req-with-address.log",
        "tests/expectations/inspect/client-req-with-address.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_client_reqres() {
    assert_equals_to_file(
        "tests/corpus/clientreqres.log",
        "tests/expectations/inspect/clientreqres.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_content_length_zero_res() {
    assert_equals_to_file(
        "tests/corpus/content-length-0-res.log",
        "tests/expectations/inspect/content-length-0-res.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_log_1() {
    assert_equals_to_file(
        "tests/corpus/log1.log",
        "tests/expectations/inspect/log1.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_log_2() {
    assert_equals_to_file(
        "tests/corpus/log2.log",
        "tests/expectations/inspect/log2.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_non_object_res() {
    assert_equals_to_file(
        "tests/corpus/non-object-res.log",
        "tests/expectations/inspect/non-object-res.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_res_header() {
    assert_equals_to_file(
        "tests/corpus/res-header.log",
        "tests/expectations/inspect/res-header.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_res_without_header() {
    assert_equals_to_file(
        "tests/corpus/res-without-header.log",
        "tests/expectations/inspect/res-without-header.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_old_crashers_139() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/139.log",
        "tests/expectations/inspect/old-crashers/139.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_old_crashers_144() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/144.log",
        "tests/expectations/inspect/old-crashers/144.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_old_crashers_233() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/233.log",
        "tests/expectations/inspect/old-crashers/233.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_old_crashers_242() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/242.log",
        "tests/expectations/inspect/old-crashers/242.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_old_crashers_244() {
    assert_equals_to_file(
        "tests/corpus/old-crashers/244.log",
        "tests/expectations/inspect/old-crashers/244.log.expected",
        LogFormat::Inspect,
    );
}

/* ============================================================================================== *\
 * Test corpus files created for rust bunyan view
 * ============================================================================================== */

#[test]
fn inspect_error_with_stack() {
    assert_equals_to_file(
        "tests/corpus/error-with-stack.log",
        "tests/expectations/inspect/error-with-stack.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_req_with_newlines() {
    assert_equals_to_file(
        "tests/corpus/req-with-newlines.log",
        "tests/expectations/inspect/req-with-newlines.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_req_with_trailers() {
    assert_equals_to_file(
        "tests/corpus/req-with-trailers.log",
        "tests/expectations/inspect/req-with-trailers.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_res_with_empty_object() {
    assert_equals_to_file(
        "tests/corpus/res-with-empty-object.log",
        "tests/expectations/inspect/res-with-empty-object.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_with_numeric_req_id() {
    assert_equals_to_file(
        "tests/corpus/with-numeric-req-id.log",
        "tests/expectations/inspect/with-numeric-req-id.log.expected",
        LogFormat::Inspect,
    );
}

#[test]
fn inspect_with_weird_extra_params() {
    assert_equals_to_file(
        "tests/corpus/with-weird-extra-params.log",
        "tests/expectations/inspect/with-weird-extra-params.log.expected",
        LogFormat::Inspect,
    );
}
