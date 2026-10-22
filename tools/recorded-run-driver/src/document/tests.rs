// SPDX-License-Identifier: MIT

//! Original synthetic syntax fixtures; they do not define owner bundle semantics.

use super::decode_document;
use serde_json::json;
use std::io::ErrorKind;

#[test]
fn unique_documents_preserve_all_json_value_kinds_and_escapes() {
    let input = br#" {"null":null,"bool":false,"signed":-7,"unsigned":18446744073709551615,
        "fraction":1.25,"text":"a\u0062","array":[null,true,{"a/b":"~"}]} "#;
    assert_eq!(
        decode_document(input).unwrap(),
        json!({
            "null": null, "bool": false, "signed": -7, "unsigned": u64::MAX,
            "fraction": 1.25, "text": "ab", "array": [null, true, {"a/b": "~"}]
        })
    );
    assert_eq!(
        decode_document(br#"{"first":{"key":1},"second":{"key":2}}"#).unwrap(),
        json!({"first":{"key":1},"second":{"key":2}})
    );
}

#[test]
fn duplicate_fields_fail_at_root_nested_objects_and_array_elements() {
    for input in [
        br#"{"valid":false,"valid":true}"#.as_slice(),
        br#"{"identity":{"value":"foreign","value":"expected"}}"#,
        br#"{"events":[{"outcome":"unknown","outcome":"settled"}]}"#,
        br#"{"null":null,"null":null}"#,
        br#"{"object":{},"object":{}}"#,
    ] {
        let error = decode_document(input).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidData);
        assert_eq!(error.to_string(), "invalid or ambiguous JSON document");
    }
}

#[test]
fn escaped_keys_are_compared_after_json_string_decoding() {
    for input in [
        br#"{"valid":false,"v\u0061lid":true}"#.as_slice(),
        br#"{"mapping":{"/value":1,"\u002fvalue":2}}"#,
        br#"[{"\u0061":1,"a":2}]"#,
    ] {
        assert_eq!(
            decode_document(input).unwrap_err().kind(),
            ErrorKind::InvalidData
        );
    }
    assert_eq!(
        decode_document(br#"{"a":1,"A":2}"#).unwrap(),
        json!({"a":1,"A":2})
    );
}

#[test]
fn malformed_trailing_and_non_utf8_input_has_no_raw_diagnostic() {
    for input in [
        b"".as_slice(),
        b"\xff",
        b"{}[]",
        b"{\"a\":1,}",
        b"[1,]",
        br#"{"SYNTHETIC_PRIVATE_FIELD":"SYNTHETIC_PRIVATE_VALUE","SYNTHETIC_PRIVATE_FIELD":null}"#,
        br#"{"SYNTHETIC_PRIVATE_FIELD":"unterminated"#,
    ] {
        let error = decode_document(input).unwrap_err();
        assert_eq!(error.to_string(), "invalid or ambiguous JSON document");
        assert!(!error.to_string().contains("SYNTHETIC_PRIVATE"));
    }
}

#[test]
fn direct_decoder_calls_retain_the_existing_byte_and_recursion_bounds() {
    let mut input = vec![b' '; crate::REPORT_LIMIT as usize];
    input[0] = b'0';
    assert_eq!(decode_document(&input).unwrap(), json!(0));
    input.push(b' ');
    assert_eq!(
        decode_document(&input).unwrap_err().to_string(),
        "JSON document exceeds report bound"
    );
    let nested = "[".repeat(256) + "0" + &"]".repeat(256);
    assert_eq!(
        decode_document(nested.as_bytes()).unwrap_err().kind(),
        ErrorKind::InvalidData
    );
}
