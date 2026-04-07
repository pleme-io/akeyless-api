use akeyless_api::models;
use serde_json::json;

// ============================================================
// double_option fields: Option<Option<serde_json::Value>>
//
// These tests verify the three-state semantics:
//   None          = field missing from JSON
//   Some(None)    = field present as JSON null
//   Some(Some(v)) = field present with value
//
// Bug class: incorrect null handling leads to data loss when
// APIs distinguish "unset" from "explicitly null".
// ============================================================

// --- UscGetSecretOutput ---

#[test]
fn usc_output_metadata_missing() {
    let j = json!({"name": "test"});
    let out: models::UscGetSecretOutput = serde_json::from_value(j).unwrap();
    assert!(out.metadata.is_none(), "missing field => None");
}

#[test]
fn usc_output_metadata_null() {
    let j = json!({"metadata": null});
    let out: models::UscGetSecretOutput = serde_json::from_value(j).unwrap();
    assert_eq!(out.metadata, Some(None), "null field => Some(None)");
}

#[test]
fn usc_output_metadata_with_value() {
    let j = json!({"metadata": {"key": "val"}});
    let out: models::UscGetSecretOutput = serde_json::from_value(j).unwrap();
    assert_eq!(
        out.metadata,
        Some(Some(json!({"key": "val"}))),
        "present value => Some(Some(...))"
    );
}

#[test]
fn usc_output_metadata_serialize_none_omits() {
    let out = models::UscGetSecretOutput {
        metadata: None,
        ..Default::default()
    };
    let j = serde_json::to_value(&out).unwrap();
    assert!(j.get("metadata").is_none(), "None should be omitted");
}

#[test]
fn usc_output_metadata_serialize_some_none_as_null() {
    let out = models::UscGetSecretOutput {
        metadata: Some(None),
        ..Default::default()
    };
    let j = serde_json::to_value(&out).unwrap();
    assert_eq!(j["metadata"], serde_json::Value::Null);
}

#[test]
fn usc_output_metadata_serialize_some_some() {
    let out = models::UscGetSecretOutput {
        metadata: Some(Some(json!({"a": 1}))),
        ..Default::default()
    };
    let j = serde_json::to_value(&out).unwrap();
    assert_eq!(j["metadata"], json!({"a": 1}));
}

#[test]
fn usc_output_roundtrip_all_states() {
    for metadata in [
        None,
        Some(None),
        Some(Some(json!("string_meta"))),
        Some(Some(json!(42))),
        Some(Some(json!([1, 2, 3]))),
    ] {
        let out = models::UscGetSecretOutput {
            metadata: metadata.clone(),
            name: Some("test".into()),
            ..Default::default()
        };
        let j = serde_json::to_string(&out).unwrap();
        let de: models::UscGetSecretOutput = serde_json::from_str(&j).unwrap();
        assert_eq!(de.metadata, metadata);
    }
}

// --- ConfigHash (two double_option fields: producers, rotators) ---

#[test]
fn config_hash_producers_missing() {
    let j = json!({"admins": "hash1"});
    let ch: models::ConfigHash = serde_json::from_value(j).unwrap();
    assert!(ch.producers.is_none());
    assert!(ch.rotators.is_none());
}

#[test]
fn config_hash_producers_null() {
    let j = json!({"producers": null, "rotators": null});
    let ch: models::ConfigHash = serde_json::from_value(j).unwrap();
    assert_eq!(ch.producers, Some(None));
    assert_eq!(ch.rotators, Some(None));
}

#[test]
fn config_hash_producers_with_value() {
    let j = json!({"producers": {"p1": "running"}, "rotators": ["r1"]});
    let ch: models::ConfigHash = serde_json::from_value(j).unwrap();
    assert_eq!(ch.producers, Some(Some(json!({"p1": "running"}))));
    assert_eq!(ch.rotators, Some(Some(json!(["r1"]))));
}

#[test]
fn config_hash_roundtrip() {
    let ch = models::ConfigHash {
        admins: Some("hash_a".into()),
        producers: Some(Some(json!({"active": true}))),
        rotators: Some(None),
        ..Default::default()
    };
    let j = serde_json::to_string(&ch).unwrap();
    let de: models::ConfigHash = serde_json::from_str(&j).unwrap();
    assert_eq!(de.producers, Some(Some(json!({"active": true}))));
    assert_eq!(de.rotators, Some(None));
    assert_eq!(de.admins.as_deref(), Some("hash_a"));
}

// --- SecretInfo (double_option on location, also has Box, HashMap, i64, bool) ---

#[test]
fn secret_info_location_missing() {
    let j = json!({"name": "secret1"});
    let si: models::SecretInfo = serde_json::from_value(j).unwrap();
    assert!(si.location.is_none());
}

#[test]
fn secret_info_location_null() {
    let j = json!({"location": null});
    let si: models::SecretInfo = serde_json::from_value(j).unwrap();
    assert_eq!(si.location, Some(None));
}

#[test]
fn secret_info_location_with_value() {
    let j = json!({"location": "us-east-1"});
    let si: models::SecretInfo = serde_json::from_value(j).unwrap();
    assert_eq!(si.location, Some(Some(json!("us-east-1"))));
}

#[test]
fn secret_info_full_roundtrip() {
    let mut tags = std::collections::HashMap::new();
    tags.insert("env".into(), "prod".into());

    let si = models::SecretInfo {
        name: Some("db-password".into()),
        version: Some(3),
        status: Some(true),
        r#type: Some("static".into()),
        location: Some(Some(json!({"region": "us-east-1"}))),
        tags: Some(tags),
        ..Default::default()
    };
    let j = serde_json::to_string(&si).unwrap();
    let de: models::SecretInfo = serde_json::from_str(&j).unwrap();
    assert_eq!(si, de);
}

#[test]
fn secret_info_type_keyword_serde() {
    let si = models::SecretInfo {
        r#type: Some("dynamic".into()),
        ..Default::default()
    };
    let j = serde_json::to_value(&si).unwrap();
    assert_eq!(j["type"], "dynamic");

    let de: models::SecretInfo = serde_json::from_value(json!({"type": "rotated"})).unwrap();
    assert_eq!(de.r#type.as_deref(), Some("rotated"));
}

// --- LastStatusInfo (Box + double_option together) ---

#[test]
fn last_status_info_default() {
    let lsi = models::LastStatusInfo::default();
    assert!(lsi.migrations_status.is_none());
    assert!(lsi.producers_errors.is_none());
    assert!(lsi.was_migrations_copied_to_new_table.is_none());
}

#[test]
fn last_status_info_producers_errors_null_vs_missing() {
    let missing: models::LastStatusInfo = serde_json::from_value(json!({})).unwrap();
    assert!(missing.producers_errors.is_none());

    let null_val: models::LastStatusInfo = serde_json::from_value(json!({"producers_errors": null})).unwrap();
    assert_eq!(null_val.producers_errors, Some(None));

    let with_val: models::LastStatusInfo = serde_json::from_value(
        json!({"producers_errors": {"err1": "timeout"}})
    ).unwrap();
    assert_eq!(with_val.producers_errors, Some(Some(json!({"err1": "timeout"}))));
}
