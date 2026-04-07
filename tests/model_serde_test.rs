use akeyless_api::models;
use serde_json::json;

// ============================================================
// Auth model: tests kebab-case rename, all-Optional fields,
//   skip_serializing_if behavior
// ============================================================

#[test]
fn auth_default_is_all_none() {
    let auth = models::Auth::default();
    let j = serde_json::to_value(&auth).unwrap();
    assert_eq!(j, json!({}), "Default Auth should serialize to empty object due to skip_serializing_if");
}

#[test]
fn auth_new_equals_default() {
    assert_eq!(models::Auth::new(), models::Auth::default());
}

#[test]
fn auth_serialize_uses_kebab_case_keys() {
    let auth = models::Auth {
        access_id: Some("p-123".into()),
        access_key: Some("secret".into()),
        access_type: Some("access_key".into()),
        ..Default::default()
    };
    let j = serde_json::to_value(&auth).unwrap();
    assert_eq!(j["access-id"], "p-123");
    assert_eq!(j["access-key"], "secret");
    assert_eq!(j["access-type"], "access_key");
    assert!(j.get("access_id").is_none(), "snake_case key should not appear");
}

#[test]
fn auth_deserialize_kebab_case() {
    let j = json!({
        "access-id": "p-abc",
        "access-type": "jwt",
        "jwt": "eyJhbGciOiJSUzI1NiJ9"
    });
    let auth: models::Auth = serde_json::from_value(j).unwrap();
    assert_eq!(auth.access_id.as_deref(), Some("p-abc"));
    assert_eq!(auth.access_type.as_deref(), Some("jwt"));
    assert_eq!(auth.jwt.as_deref(), Some("eyJhbGciOiJSUzI1NiJ9"));
    assert!(auth.access_key.is_none());
}

#[test]
fn auth_roundtrip() {
    let auth = models::Auth {
        access_id: Some("p-roundtrip".into()),
        debug: Some(true),
        oci_group_ocid: Some(vec!["ocid1".into(), "ocid2".into()]),
        ..Default::default()
    };
    let serialized = serde_json::to_string(&auth).unwrap();
    let deserialized: models::Auth = serde_json::from_str(&serialized).unwrap();
    assert_eq!(auth, deserialized);
}

#[test]
fn auth_deserialize_ignores_unknown_fields() {
    let j = json!({
        "access-id": "p-test",
        "unknown-future-field": "value"
    });
    let auth: models::Auth = serde_json::from_value(j).unwrap();
    assert_eq!(auth.access_id.as_deref(), Some("p-test"));
}

#[test]
fn auth_empty_json_object_deserializes() {
    let auth: models::Auth = serde_json::from_value(json!({})).unwrap();
    assert_eq!(auth, models::Auth::default());
}

// ============================================================
// GetSecretValue: required `names` field (Vec<String>)
// ============================================================

#[test]
fn get_secret_value_new_sets_names() {
    let gsv = models::GetSecretValue::new(vec!["secret1".into()]);
    assert_eq!(gsv.names, vec!["secret1".to_string()]);
    assert!(gsv.token.is_none());
    assert!(gsv.version.is_none());
}

#[test]
fn get_secret_value_serialize_includes_required_names() {
    let gsv = models::GetSecretValue::new(vec!["/path/secret".into()]);
    let j = serde_json::to_value(&gsv).unwrap();
    assert_eq!(j["names"], json!(["/path/secret"]));
    assert!(j.get("token").is_none());
}

#[test]
fn get_secret_value_deserialize_missing_names_fails() {
    let j = json!({"token": "tok"});
    let result = serde_json::from_value::<models::GetSecretValue>(j);
    assert!(result.is_err(), "names is required, deserialize should fail without it");
}

#[test]
fn get_secret_value_deserialize_empty_names() {
    let j = json!({"names": []});
    let gsv: models::GetSecretValue = serde_json::from_value(j).unwrap();
    assert!(gsv.names.is_empty());
}

#[test]
fn get_secret_value_roundtrip_with_all_fields() {
    let gsv = models::GetSecretValue {
        accessibility: Some("personal".into()),
        ignore_cache: Some("true".into()),
        json: Some(true),
        names: vec!["/a".into(), "/b".into()],
        pretty_print: Some(false),
        token: Some("t-abc".into()),
        uid_token: Some("uid-xyz".into()),
        version: Some(-3),
    };
    let serialized = serde_json::to_string(&gsv).unwrap();
    let deserialized: models::GetSecretValue = serde_json::from_str(&serialized).unwrap();
    assert_eq!(gsv, deserialized);
}

#[test]
fn get_secret_value_version_negative() {
    let j = json!({"names": ["s"], "version": -5});
    let gsv: models::GetSecretValue = serde_json::from_value(j).unwrap();
    assert_eq!(gsv.version, Some(-5));
}

// ============================================================
// CreateSecret: required `name` + `value`, HashMap field,
//   Rust keyword `type` as r#type
// ============================================================

#[test]
fn create_secret_required_fields() {
    let cs = models::CreateSecret::new("my-secret".into(), "my-value".into());
    assert_eq!(cs.name, "my-secret");
    assert_eq!(cs.value, "my-value");
    assert!(cs.r#type.is_none());
}

#[test]
fn create_secret_missing_name_fails() {
    let j = json!({"value": "v"});
    let result = serde_json::from_value::<models::CreateSecret>(j);
    assert!(result.is_err());
}

#[test]
fn create_secret_missing_value_fails() {
    let j = json!({"name": "n"});
    let result = serde_json::from_value::<models::CreateSecret>(j);
    assert!(result.is_err());
}

#[test]
fn create_secret_type_field_serde() {
    let cs = models::CreateSecret {
        r#type: Some("generic".into()),
        ..models::CreateSecret::new("n".into(), "v".into())
    };
    let j = serde_json::to_value(&cs).unwrap();
    assert_eq!(j["type"], "generic");
}

#[test]
fn create_secret_custom_field_hashmap() {
    let mut hm = std::collections::HashMap::new();
    hm.insert("env".into(), "prod".into());
    hm.insert("team".into(), "platform".into());
    let cs = models::CreateSecret {
        custom_field: Some(hm.clone()),
        ..models::CreateSecret::new("s".into(), "v".into())
    };
    let j = serde_json::to_value(&cs).unwrap();
    assert_eq!(j["custom-field"]["env"], "prod");
    assert_eq!(j["custom-field"]["team"], "platform");

    let deserialized: models::CreateSecret = serde_json::from_value(j).unwrap();
    assert_eq!(deserialized.custom_field.unwrap(), hm);
}

#[test]
fn create_secret_tags_vec() {
    let cs = models::CreateSecret {
        tags: Some(vec!["env:prod".into(), "team:infra".into()]),
        ..models::CreateSecret::new("s".into(), "v".into())
    };
    let j = serde_json::to_value(&cs).unwrap();
    assert_eq!(j["tags"], json!(["env:prod", "team:infra"]));
}

#[test]
fn create_secret_roundtrip() {
    let cs = models::CreateSecret {
        name: "test".into(),
        value: "secret_val".into(),
        description: Some("A test secret".into()),
        format: Some("json".into()),
        tags: Some(vec!["t1".into()]),
        r#type: Some("generic".into()),
        ..models::CreateSecret::new("test".into(), "secret_val".into())
    };
    let serialized = serde_json::to_string(&cs).unwrap();
    let deserialized: models::CreateSecret = serde_json::from_str(&serialized).unwrap();
    assert_eq!(cs, deserialized);
}

// ============================================================
// AuthOutput: Box<models::T> field
// ============================================================

#[test]
fn auth_output_default_all_none() {
    let ao = models::AuthOutput::default();
    assert!(ao.token.is_none());
    assert!(ao.creds.is_none());
    assert!(ao.expiration.is_none());
}

#[test]
fn auth_output_serialize_with_token() {
    let ao = models::AuthOutput {
        token: Some("t-abcdef".into()),
        ..Default::default()
    };
    let j = serde_json::to_value(&ao).unwrap();
    assert_eq!(j["token"], "t-abcdef");
    assert!(j.get("creds").is_none());
}

#[test]
fn auth_output_with_boxed_creds() {
    let creds = models::SystemAccessCredentialsReplyObj {
        access_id: Some("p-cred".into()),
        token: Some("cred-tok".into()),
        expiry: Some(1700000000),
        ..Default::default()
    };
    let ao = models::AuthOutput {
        creds: Some(Box::new(creds)),
        token: Some("main-tok".into()),
        ..Default::default()
    };
    let j = serde_json::to_value(&ao).unwrap();
    assert_eq!(j["creds"]["access_id"], "p-cred");
    assert_eq!(j["creds"]["token"], "cred-tok");
    assert_eq!(j["creds"]["expiry"], 1700000000);

    let deserialized: models::AuthOutput = serde_json::from_value(j).unwrap();
    assert_eq!(deserialized.creds.unwrap().access_id.as_deref(), Some("p-cred"));
}

// ============================================================
// JsonError
// ============================================================

#[test]
fn json_error_default() {
    let je = models::JsonError::default();
    assert!(je.error.is_none());
}

#[test]
fn json_error_roundtrip() {
    let je = models::JsonError { error: Some("unauthorized".into()) };
    let j = serde_json::to_value(&je).unwrap();
    assert_eq!(j["error"], "unauthorized");
    let de: models::JsonError = serde_json::from_value(j).unwrap();
    assert_eq!(de, je);
}

#[test]
fn json_error_from_api_response() {
    let raw = r#"{"error": "token expired"}"#;
    let je: models::JsonError = serde_json::from_str(raw).unwrap();
    assert_eq!(je.error.as_deref(), Some("token expired"));
}

// ============================================================
// ValidateToken: small model
// ============================================================

#[test]
fn validate_token_default() {
    let vt = models::ValidateToken::default();
    assert!(vt.json.is_none());
    assert!(vt.token.is_none());
}

#[test]
fn validate_token_roundtrip() {
    let vt = models::ValidateToken {
        json: Some(true),
        token: Some("tok_123".into()),
    };
    let serialized = serde_json::to_string(&vt).unwrap();
    let deserialized: models::ValidateToken = serde_json::from_str(&serialized).unwrap();
    assert_eq!(vt, deserialized);
}
