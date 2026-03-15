# LetsEncryptTargetDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_key_pem** | Option<**String**> | ACME Account Private Key (PEM-encoded) Supports ECDSA (P-256, P-384, P-521), RSA (2048+), and Ed25519 Auto-generated as ECDSA P-256 on first certificate issuance if not provided Stored encrypted, required for certificate operations and revocation | [optional]
**account_url** | Option<**String**> | ACME Account URL (returned after registration with Let's Encrypt) Used to retrieve existing account instead of re-registering | [optional]
**acme_environment** | Option<**String**> | ACMEEnvironment defines Let's Encrypt ACME directory environment | [optional]
**challenge_type** | Option<**String**> | ACMEChallengeType defines ACME challenge type for Let's Encrypt | [optional]
**dns_target_name** | Option<**String**> | Name of DNS target (transient field - not stored in DB) Used by CLI to pass DNS target name to SDK for creating target_object_assoc Retrieved from target_object_assoc when reading target Required when ChallengeType is \"dns\" | [optional]
**dns_target_type** | Option<**String**> |  | [optional]
**email** | Option<**String**> | Email address for ACME account registration Required | [optional]
**gcp_project** | Option<**String**> | GCP Cloud DNS: Project ID Optional - can be derived from service account | [optional]
**hosted_zone** | Option<**String**> | AWS Route53: Hosted zone ID Required when DNSTargetType is AWS | [optional]
**resource_group** | Option<**String**> | Azure DNS: Resource group name Required when DNSTargetType is Azure | [optional]
**timeout** | Option<**i64**> | A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


