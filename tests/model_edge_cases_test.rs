use akeyless_api::models;
use serde_json::json;

// ============================================================
// Edge cases that catch real bugs:
// - Empty inputs
// - Large payloads
// - Special characters in string fields
// - Numeric boundary values
// - Vec/HashMap emptiness
// - Box<T> nesting
// - Partial JSON deserialization
// ============================================================

// --- Empty and minimal inputs ---

#[test]
fn auth_from_empty_json() {
    let auth: models::Auth = serde_json::from_value(json!({})).unwrap();
    assert_eq!(auth, models::Auth::default());
}

#[test]
fn get_secret_value_empty_names_vec() {
    let gsv = models::GetSecretValue::new(vec![]);
    let j = serde_json::to_value(&gsv).unwrap();
    assert_eq!(j["names"], json!([]));
    let de: models::GetSecretValue = serde_json::from_value(j).unwrap();
    assert!(de.names.is_empty());
}

#[test]
fn get_secret_value_many_names() {
    let names: Vec<String> = (0..100).map(|i| format!("/secrets/key-{}", i)).collect();
    let gsv = models::GetSecretValue::new(names.clone());
    let j = serde_json::to_string(&gsv).unwrap();
    let de: models::GetSecretValue = serde_json::from_str(&j).unwrap();
    assert_eq!(de.names.len(), 100);
    assert_eq!(de.names[99], "/secrets/key-99");
}

// --- Special characters in string fields ---

#[test]
fn auth_access_id_special_chars() {
    let auth = models::Auth {
        access_id: Some("p-特殊文字/test@#$%^&*()".into()),
        ..Default::default()
    };
    let j = serde_json::to_string(&auth).unwrap();
    let de: models::Auth = serde_json::from_str(&j).unwrap();
    assert_eq!(de.access_id.as_deref(), Some("p-特殊文字/test@#$%^&*()"));
}

#[test]
fn create_secret_value_with_newlines_and_tabs() {
    let cs = models::CreateSecret::new(
        "multiline".into(),
        "line1\nline2\ttab\rcarriage".into(),
    );
    let j = serde_json::to_string(&cs).unwrap();
    let de: models::CreateSecret = serde_json::from_str(&j).unwrap();
    assert_eq!(de.value, "line1\nline2\ttab\rcarriage");
}

#[test]
fn create_secret_value_with_json_string() {
    let inner_json = r#"{"nested": "json", "array": [1,2,3]}"#;
    let cs = models::CreateSecret::new("json-secret".into(), inner_json.into());
    let j = serde_json::to_string(&cs).unwrap();
    let de: models::CreateSecret = serde_json::from_str(&j).unwrap();
    assert_eq!(de.value, inner_json);
}

#[test]
fn create_secret_value_empty_string() {
    let cs = models::CreateSecret::new("empty".into(), "".into());
    let j = serde_json::to_string(&cs).unwrap();
    let de: models::CreateSecret = serde_json::from_str(&j).unwrap();
    assert_eq!(de.value, "");
}

// --- Numeric boundary values ---

#[test]
fn get_secret_value_version_i32_boundaries() {
    for version in [i32::MIN, -1, 0, 1, i32::MAX] {
        let gsv = models::GetSecretValue {
            version: Some(version),
            ..models::GetSecretValue::new(vec!["s".into()])
        };
        let j = serde_json::to_string(&gsv).unwrap();
        let de: models::GetSecretValue = serde_json::from_str(&j).unwrap();
        assert_eq!(de.version, Some(version));
    }
}

#[test]
fn secret_info_version_i64_boundaries() {
    for version in [i64::MIN, -1i64, 0i64, 1i64, i64::MAX] {
        let si = models::SecretInfo {
            version: Some(version),
            ..Default::default()
        };
        let j = serde_json::to_string(&si).unwrap();
        let de: models::SecretInfo = serde_json::from_str(&j).unwrap();
        assert_eq!(de.version, Some(version));
    }
}

#[test]
fn system_access_creds_expiry_i64() {
    let creds = models::SystemAccessCredentialsReplyObj {
        expiry: Some(0),
        ..Default::default()
    };
    let j = serde_json::to_string(&creds).unwrap();
    let de: models::SystemAccessCredentialsReplyObj = serde_json::from_str(&j).unwrap();
    assert_eq!(de.expiry, Some(0));

    let creds_max = models::SystemAccessCredentialsReplyObj {
        expiry: Some(i64::MAX),
        ..Default::default()
    };
    let j2 = serde_json::to_string(&creds_max).unwrap();
    let de2: models::SystemAccessCredentialsReplyObj = serde_json::from_str(&j2).unwrap();
    assert_eq!(de2.expiry, Some(i64::MAX));
}

// --- HashMap edge cases ---

#[test]
fn create_secret_empty_hashmap() {
    let cs = models::CreateSecret {
        custom_field: Some(std::collections::HashMap::new()),
        ..models::CreateSecret::new("s".into(), "v".into())
    };
    let j = serde_json::to_value(&cs).unwrap();
    assert_eq!(j["custom-field"], json!({}));
    let de: models::CreateSecret = serde_json::from_value(j).unwrap();
    assert!(de.custom_field.unwrap().is_empty());
}

#[test]
fn secret_info_tags_hashmap() {
    let mut tags = std::collections::HashMap::new();
    tags.insert("".into(), "empty-key".into());
    tags.insert("key-with-special-chars!@#".into(), "val".into());

    let si = models::SecretInfo {
        tags: Some(tags.clone()),
        ..Default::default()
    };
    let j = serde_json::to_string(&si).unwrap();
    let de: models::SecretInfo = serde_json::from_str(&j).unwrap();
    assert_eq!(de.tags.unwrap(), tags);
}

// --- Vec edge cases ---

#[test]
fn auth_oci_group_ocid_empty_vec() {
    let auth = models::Auth {
        oci_group_ocid: Some(vec![]),
        ..Default::default()
    };
    let j = serde_json::to_value(&auth).unwrap();
    assert_eq!(j["oci-group-ocid"], json!([]));
}

#[test]
fn create_secret_inject_url_vec() {
    let cs = models::CreateSecret {
        inject_url: Some(vec![
            "https://example.com".into(),
            "https://other.com/path?q=1&r=2".into(),
        ]),
        ..models::CreateSecret::new("n".into(), "v".into())
    };
    let j = serde_json::to_string(&cs).unwrap();
    let de: models::CreateSecret = serde_json::from_str(&j).unwrap();
    assert_eq!(de.inject_url.unwrap().len(), 2);
}

// --- Box<T> nesting ---

#[test]
fn auth_output_boxed_creds_roundtrip() {
    let creds = models::SystemAccessCredentialsReplyObj {
        access_id: Some("p-nested".into()),
        auth_creds: Some("auth".into()),
        kfm_creds: Some("kfm".into()),
        uam_creds: Some("uam".into()),
        token: Some("tok".into()),
        expiry: Some(999),
        need_mfa_app_first_config: Some(false),
        required_mfa: Some("totp".into()),
    };
    let ao = models::AuthOutput {
        creds: Some(Box::new(creds)),
        token: Some("main".into()),
        expiration: Some("2025-01-01T00:00:00Z".into()),
        complete_auth_link: Some("https://auth.example.com".into()),
    };
    let j = serde_json::to_string(&ao).unwrap();
    let de: models::AuthOutput = serde_json::from_str(&j).unwrap();
    assert_eq!(ao, de);
}

#[test]
fn gateway_update_producer_aws_output_boxed_none() {
    let out = models::GatewayUpdateProducerAwsOutput::default();
    assert!(out.producer_details.is_none());
    let j = serde_json::to_value(&out).unwrap();
    assert_eq!(j, json!({}));
}

// --- Partial deserialization (only known fields) ---

#[test]
fn partial_json_with_extra_fields() {
    let j = json!({
        "token": "tok_partial",
        "complete_auth_link": "https://link",
        "future_field_1": 42,
        "future_field_2": {"nested": true}
    });
    let ao: models::AuthOutput = serde_json::from_value(j).unwrap();
    assert_eq!(ao.token.as_deref(), Some("tok_partial"));
    assert_eq!(ao.complete_auth_link.as_deref(), Some("https://link"));
    assert!(ao.creds.is_none());
}

// --- Clone and PartialEq ---

#[test]
fn model_clone_and_equality() {
    let auth = models::Auth {
        access_id: Some("p-clone".into()),
        access_type: Some("access_key".into()),
        ..Default::default()
    };
    let cloned = auth.clone();
    assert_eq!(auth, cloned);
}

#[test]
fn model_inequality() {
    let a1 = models::Auth {
        access_id: Some("p-1".into()),
        ..Default::default()
    };
    let a2 = models::Auth {
        access_id: Some("p-2".into()),
        ..Default::default()
    };
    assert_ne!(a1, a2);
}

// --- Default trait ---

#[test]
fn default_trait_consistency() {
    assert_eq!(models::Auth::new(), models::Auth::default());
    assert_eq!(models::AuthOutput::new(), models::AuthOutput::default());
    assert_eq!(models::JsonError::new(), models::JsonError::default());
    assert_eq!(models::ValidateToken::new(), models::ValidateToken::default());
    assert_eq!(models::UscGetSecretOutput::new(), models::UscGetSecretOutput::default());
    assert_eq!(models::ConfigHash::new(), models::ConfigHash::default());
    assert_eq!(models::SecretInfo::new(), models::SecretInfo::default());
    assert_eq!(models::LastStatusInfo::new(), models::LastStatusInfo::default());
    assert_eq!(models::GithubMetadata::new(), models::GithubMetadata::default());
    assert_eq!(
        models::SystemAccessCredentialsReplyObj::new(),
        models::SystemAccessCredentialsReplyObj::default()
    );
    assert_eq!(
        models::GatewayUpdateProducerAwsOutput::new(),
        models::GatewayUpdateProducerAwsOutput::default()
    );
}

// --- SecretInfo with Box<GithubMetadata> ---

#[test]
fn secret_info_with_github_metadata() {
    let gh = models::GithubMetadata {
        organization_name: Some("my-org".into()),
        repository: Some("my-repo".into()),
        scope: Some("repo".into()),
        ..Default::default()
    };
    let si = models::SecretInfo {
        github: Some(Box::new(gh)),
        name: Some("gh-secret".into()),
        ..Default::default()
    };
    let j = serde_json::to_value(&si).unwrap();
    assert_eq!(j["github"]["organization_name"], "my-org");
    assert_eq!(j["github"]["repository"], "my-repo");

    let de: models::SecretInfo = serde_json::from_value(j).unwrap();
    assert_eq!(de.github.unwrap().organization_name.as_deref(), Some("my-org"));
}

// --- GithubMetadata ---

#[test]
fn github_metadata_roundtrip() {
    let gh = models::GithubMetadata {
        environment_name: Some("production".into()),
        organization_name: Some("acme".into()),
        repository: Some("acme/app".into()),
        repository_access: Some("read".into()),
        scope: Some("repo,org".into()),
        selected_repositories: Some("repo1,repo2".into()),
    };
    let j = serde_json::to_string(&gh).unwrap();
    let de: models::GithubMetadata = serde_json::from_str(&j).unwrap();
    assert_eq!(gh, de);
}
