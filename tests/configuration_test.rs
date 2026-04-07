use akeyless_api::apis::configuration::{Configuration, ApiKey};

#[test]
fn default_base_path() {
    let config = Configuration::default();
    assert_eq!(config.base_path, "https://api.akeyless.io");
}

#[test]
fn default_user_agent() {
    let config = Configuration::default();
    assert_eq!(config.user_agent, Some("OpenAPI-Generator/3.0/rust".to_string()));
}

#[test]
fn default_no_auth() {
    let config = Configuration::default();
    assert!(config.basic_auth.is_none());
    assert!(config.oauth_access_token.is_none());
    assert!(config.bearer_access_token.is_none());
    assert!(config.api_key.is_none());
}

#[test]
fn new_equals_default() {
    let c1 = Configuration::new();
    let c2 = Configuration::default();
    assert_eq!(c1.base_path, c2.base_path);
    assert_eq!(c1.user_agent, c2.user_agent);
    assert_eq!(c1.basic_auth, c2.basic_auth);
    assert_eq!(c1.oauth_access_token, c2.oauth_access_token);
    assert_eq!(c1.bearer_access_token, c2.bearer_access_token);
}

#[test]
fn configuration_is_clone() {
    let mut config = Configuration::new();
    config.base_path = "https://custom.example.com".to_string();
    config.bearer_access_token = Some("tok_abc".to_string());
    let cloned = config.clone();
    assert_eq!(cloned.base_path, "https://custom.example.com");
    assert_eq!(cloned.bearer_access_token, Some("tok_abc".to_string()));
}

#[test]
fn configuration_debug_does_not_panic() {
    let config = Configuration::new();
    let debug = format!("{:?}", config);
    assert!(debug.contains("Configuration"));
}

#[test]
fn custom_base_path() {
    let mut config = Configuration::new();
    config.base_path = "https://eu.akeyless.io".to_string();
    assert_eq!(config.base_path, "https://eu.akeyless.io");
}

#[test]
fn basic_auth_with_password() {
    let mut config = Configuration::new();
    config.basic_auth = Some(("user".to_string(), Some("pass".to_string())));
    let (user, pass) = config.basic_auth.unwrap();
    assert_eq!(user, "user");
    assert_eq!(pass, Some("pass".to_string()));
}

#[test]
fn basic_auth_without_password() {
    let mut config = Configuration::new();
    config.basic_auth = Some(("user".to_string(), None));
    let (user, pass) = config.basic_auth.unwrap();
    assert_eq!(user, "user");
    assert!(pass.is_none());
}

#[test]
fn api_key_with_prefix() {
    let key = ApiKey {
        prefix: Some("Bearer".to_string()),
        key: "abc123".to_string(),
    };
    assert_eq!(key.prefix, Some("Bearer".to_string()));
    assert_eq!(key.key, "abc123");
}

#[test]
fn api_key_without_prefix() {
    let key = ApiKey {
        prefix: None,
        key: "raw_key".to_string(),
    };
    assert!(key.prefix.is_none());
    assert_eq!(key.key, "raw_key");
}

#[test]
fn api_key_clone_and_debug() {
    let key = ApiKey {
        prefix: Some("Token".to_string()),
        key: "secret".to_string(),
    };
    let cloned = key.clone();
    assert_eq!(cloned.key, "secret");
    let debug = format!("{:?}", key);
    assert!(debug.contains("ApiKey"));
}

#[test]
fn configuration_all_auth_fields() {
    let mut config = Configuration::new();
    config.basic_auth = Some(("u".to_string(), Some("p".to_string())));
    config.oauth_access_token = Some("oauth_tok".to_string());
    config.bearer_access_token = Some("bearer_tok".to_string());
    config.api_key = Some(ApiKey { prefix: None, key: "k".to_string() });
    assert!(config.basic_auth.is_some());
    assert!(config.oauth_access_token.is_some());
    assert!(config.bearer_access_token.is_some());
    assert!(config.api_key.is_some());
}
