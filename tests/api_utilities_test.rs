use akeyless_api::apis::{urlencode, parse_deep_object, Error, ResponseContent};
use serde_json::json;

#[test]
fn urlencode_plain_ascii() {
    assert_eq!(urlencode("hello"), "hello");
}

#[test]
fn urlencode_spaces_become_plus() {
    assert_eq!(urlencode("hello world"), "hello+world");
}

#[test]
fn urlencode_special_characters() {
    let encoded = urlencode("key=value&foo=bar");
    assert_eq!(encoded, "key%3Dvalue%26foo%3Dbar");
}

#[test]
fn urlencode_unicode() {
    let encoded = urlencode("café");
    assert!(!encoded.contains('é'));
    assert!(encoded.contains("caf"));
}

#[test]
fn urlencode_empty_string() {
    assert_eq!(urlencode(""), "");
}

#[test]
fn urlencode_slashes_and_colons() {
    let encoded = urlencode("https://api.akeyless.io/v2/auth");
    assert!(encoded.contains("%3A"));
    assert!(encoded.contains("%2F"));
}

#[test]
fn urlencode_already_encoded_percent() {
    let encoded = urlencode("100%25");
    assert!(encoded.contains("%25"));
}

#[test]
fn parse_deep_object_flat_strings() {
    let value = json!({"name": "alice", "role": "admin"});
    let mut params = parse_deep_object("filter", &value);
    params.sort();
    assert_eq!(params, vec![
        ("filter[name]".to_string(), "alice".to_string()),
        ("filter[role]".to_string(), "admin".to_string()),
    ]);
}

#[test]
fn parse_deep_object_nested_object() {
    let value = json!({"outer": {"inner": "val"}});
    let params = parse_deep_object("q", &value);
    assert_eq!(params, vec![
        ("q[outer][inner]".to_string(), "val".to_string()),
    ]);
}

#[test]
#[should_panic(expected = "Only objects are supported")]
fn parse_deep_object_array_of_primitives_panics() {
    let value = json!({"tags": ["a", "b"]});
    parse_deep_object("filter", &value);
}

#[test]
fn parse_deep_object_array_of_objects() {
    let value = json!({"tags": [{"name": "a"}, {"name": "b"}]});
    let params = parse_deep_object("filter", &value);
    assert_eq!(params, vec![
        ("filter[tags][0][name]".to_string(), "a".to_string()),
        ("filter[tags][1][name]".to_string(), "b".to_string()),
    ]);
}

#[test]
fn parse_deep_object_numeric_values() {
    let value = json!({"count": 42, "enabled": true});
    let mut params = parse_deep_object("cfg", &value);
    params.sort();
    assert_eq!(params, vec![
        ("cfg[count]".to_string(), "42".to_string()),
        ("cfg[enabled]".to_string(), "true".to_string()),
    ]);
}

#[test]
fn parse_deep_object_null_value() {
    let value = json!({"key": null});
    let params = parse_deep_object("p", &value);
    assert_eq!(params, vec![
        ("p[key]".to_string(), "null".to_string()),
    ]);
}

#[test]
fn parse_deep_object_empty_object() {
    let value = json!({});
    let params = parse_deep_object("p", &value);
    assert!(params.is_empty());
}

#[test]
fn parse_deep_object_deeply_nested() {
    let value = json!({"a": {"b": {"c": "deep"}}});
    let params = parse_deep_object("root", &value);
    assert_eq!(params, vec![
        ("root[a][b][c]".to_string(), "deep".to_string()),
    ]);
}

#[test]
fn parse_deep_object_mixed_array_with_objects() {
    let value = json!({"items": [{"id": "1"}, {"id": "2"}]});
    let params = parse_deep_object("data", &value);
    assert_eq!(params, vec![
        ("data[items][0][id]".to_string(), "1".to_string()),
        ("data[items][1][id]".to_string(), "2".to_string()),
    ]);
}

#[test]
#[should_panic(expected = "Only objects are supported")]
fn parse_deep_object_non_object_panics() {
    parse_deep_object("p", &json!("string_value"));
}

#[test]
#[should_panic(expected = "Only objects are supported")]
fn parse_deep_object_array_at_root_panics() {
    parse_deep_object("p", &json!([1, 2, 3]));
}

#[test]
fn error_display_reqwest_variant() {
    let client = reqwest::Client::new();
    let err = client.get("://invalid").build().unwrap_err();
    let api_err: Error<()> = Error::Reqwest(err);
    let msg = format!("{}", api_err);
    assert!(msg.starts_with("error in reqwest:"));
}

#[test]
fn error_display_serde_variant() {
    let serde_err = serde_json::from_str::<serde_json::Value>("not json").unwrap_err();
    let api_err: Error<()> = Error::Serde(serde_err);
    let msg = format!("{}", api_err);
    assert!(msg.starts_with("error in serde:"));
}

#[test]
fn error_display_io_variant() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
    let api_err: Error<()> = Error::Io(io_err);
    let msg = format!("{}", api_err);
    assert!(msg.contains("IO"));
    assert!(msg.contains("file missing"));
}

#[test]
fn error_display_response_error_variant() {
    let rc = ResponseContent {
        status: reqwest::StatusCode::FORBIDDEN,
        content: "forbidden".to_string(),
        entity: Some("details"),
    };
    let api_err: Error<&str> = Error::ResponseError(rc);
    let msg = format!("{}", api_err);
    assert!(msg.contains("response"));
    assert!(msg.contains("403"));
}

#[test]
fn error_source_reqwest_returns_some() {
    use std::error::Error as StdError;
    let client = reqwest::Client::new();
    let err = client.get("://invalid").build().unwrap_err();
    let api_err: Error<()> = Error::Reqwest(err);
    assert!(api_err.source().is_some());
}

#[test]
fn error_source_response_error_returns_none() {
    use std::error::Error as StdError;
    let rc = ResponseContent {
        status: reqwest::StatusCode::BAD_REQUEST,
        content: "bad".to_string(),
        entity: None::<()>,
    };
    let api_err: Error<()> = Error::ResponseError(rc);
    assert!(api_err.source().is_none());
}

#[test]
fn error_from_reqwest() {
    let client = reqwest::Client::new();
    let err = client.get("://invalid").build().unwrap_err();
    let api_err: Error<()> = Error::from(err);
    assert!(matches!(api_err, Error::Reqwest(_)));
}

#[test]
fn error_from_serde() {
    let err = serde_json::from_str::<()>("bad").unwrap_err();
    let api_err: Error<()> = Error::from(err);
    assert!(matches!(api_err, Error::Serde(_)));
}

#[test]
fn error_from_io() {
    let err = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "broken");
    let api_err: Error<()> = Error::from(err);
    assert!(matches!(api_err, Error::Io(_)));
}

#[test]
fn response_content_clone() {
    let rc = ResponseContent {
        status: reqwest::StatusCode::OK,
        content: "ok".to_string(),
        entity: Some(42u32),
    };
    let rc2 = rc.clone();
    assert_eq!(rc2.status, reqwest::StatusCode::OK);
    assert_eq!(rc2.content, "ok");
    assert_eq!(rc2.entity, Some(42u32));
}

#[test]
fn response_content_debug() {
    let rc = ResponseContent {
        status: reqwest::StatusCode::NOT_FOUND,
        content: "not found".to_string(),
        entity: None::<String>,
    };
    let debug = format!("{:?}", rc);
    assert!(debug.contains("404"));
    assert!(debug.contains("not found"));
}
