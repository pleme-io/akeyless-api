# akeyless-api

Auto-generated Rust SDK for the Akeyless API. 604 endpoints, 1334 types,
fully typed with serde. Generated from the official OpenAPI 3.0 spec using
`openapi-generator-cli`.

## Generation

The SDK is generated from the OpenAPI spec bundled with the Go SDK:

```bash
# Source spec
SOURCE=~/code/github/akeylesslabs/akeyless-go/api/openapi.yaml

# Convert YAML → JSON (handles datetime edge cases)
python3 -c "
import yaml, json
def default_handler(obj): return str(obj)
with open('$SOURCE') as f: spec = yaml.safe_load(f)
with open('/tmp/akeyless-openapi.json', 'w') as f:
    json.dump(spec, f, indent=2, default=default_handler)
"

# Generate Rust SDK
nix shell nixpkgs#openapi-generator-cli -c openapi-generator-cli generate \
  -i /tmp/akeyless-openapi.json \
  -g rust \
  -o . \
  --additional-properties=packageName=akeyless-api,packageVersion=0.1.0,library=reqwest
```

### Generator Details

| Setting | Value |
|---------|-------|
| Generator | `openapi-generator-cli` v7.18.0 |
| Target | `rust` (reqwest client library) |
| Source spec | `akeylesslabs/akeyless-go/api/openapi.yaml` |
| OpenAPI version | 3.0.0 |
| API version | 3.0 |
| TLS | rustls (default), native-tls optional |

### Why openapi-generator, not progenitor?

Progenitor (Oxide Computer's Rust-native generator) panics on the Akeyless spec
due to endpoints with multiple response types (`assertion failed: response_types.len() <= 1`).
openapi-generator-cli handles the full spec correctly. This is the same generator
Akeyless uses for their official Go/Python/JS SDKs (v7.10.0 for Go).

### Regenerating

When Akeyless publishes a new API version:
1. `cd ~/code/github/akeylesslabs/akeyless-go && git pull`
2. Run the generation commands above
3. `cargo check` to verify
4. Commit, push, update nix flake

tend watches `akeylesslabs/akeyless-go` for new tags — when the Go SDK updates,
the spec may have changed.

## Architecture

```
src/
├── apis/          # 604 endpoint methods (one file per tag/group)
│   ├── v2_api.rs  # Main API (all endpoints)
│   ├── configuration.rs  # Client configuration (base URL, auth)
│   └── mod.rs
├── models/        # 1334 request/response types (serde structs)
│   ├── auth.rs
│   ├── create_secret.rs
│   ├── get_secret_value.rs
│   ├── ...
│   └── mod.rs
└── lib.rs         # Crate root
```

## Usage

```rust
use akeyless_api::apis::configuration::Configuration;
use akeyless_api::apis::v2_api;
use akeyless_api::models;

let mut config = Configuration::new();
config.base_path = "https://api.akeyless.io".to_string();

// Authenticate
let auth = models::Auth {
    access_id: Some("p-xxxxxxxx".to_string()),
    access_key: Some("secret-key".to_string()),
    access_type: Some("access_key".to_string()),
    ..Default::default()
};
let auth_output = v2_api::auth(&config, auth).await?;
let token = auth_output.token.unwrap();

// Get a secret
let req = models::GetSecretValue {
    names: Some(vec!["/pleme/prod/db-password".to_string()]),
    token: Some(token),
    ..Default::default()
};
let value = v2_api::get_secret_value(&config, req).await?;
```

## Stats

- **1,177** Rust source files
- **~104,000** lines of generated code
- **604** API endpoints
- **1,334** typed models (request/response structs)
- Compiles in ~37 seconds
