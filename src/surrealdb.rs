use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use surrealdb::RecordId;

/// Decodes a JWT payload without any signature or timestamp validation.
///
/// # Errors
/// This function will return an error if:
/// - The token does not have three parts separated by dots.
/// - The payload is not valid Base64Url.
/// - The decoded payload is not valid JSON or doesn't match the Claims struct.
pub fn decode_payload_insecurely<T>(
    token: &str,
) -> Result<SurrealJWTClaims<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned + Serialize,
{
    let mut parts = token.split('.');

    let payload_b64 = parts.nth(1).ok_or("Invalid JWT format: missing payload")?;

    let decoded_payload_bytes = URL_SAFE_NO_PAD.decode(payload_b64)?;

    let claims: SurrealJWTClaims<T> = serde_json::from_slice(&decoded_payload_bytes)?;

    Ok(claims)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>", serialize = "T: Serialize"))]
/// Represents a set of JWT-like claims used by SurrealDB for authentication and authorization.
///
/// - `iat`: Issued At — Unix timestamp (seconds) when the token was created.
/// - `nbf`: Not Before — Unix timestamp before which the token MUST NOT be accepted.
/// - `exp`: Expiration Time — Unix timestamp after which the token is no longer valid.
/// - `iss`: Issuer — Identifier of the entity that issued the token (e.g., service or authority).
/// - `jti`: JWT ID — Unique identifier for the token to support revocation or deduplication.
/// - `ns` (serialized as "NS"): Namespace — SurrealDB namespace the token grants access to.
/// - `db` (serialized as "DB"): Database — SurrealDB database the token grants access to.
/// - `ac` (serialized as "AC"): Access Claims — Generic payload containing permissions/roles; type `T` allows flexibility
///   (for example, a list of permissions, a map of scopes, or a custom claims struct).
/// - `id` (serialized as "ID"): Subject Identifier — Identifier of the subject (user or service) the token represents.
///
/// All timestamps are expected to be seconds since the Unix epoch. The `NS`, `DB`, `AC`, and `ID` serde renames
/// ensure compatibility with SurrealDB's expected JSON field names.
pub struct SurrealJWTClaims<T> {
    pub iat: u64,
    pub nbf: u64,
    pub exp: u64,
    pub iss: String,
    pub jti: String,
    #[serde(rename = "NS")]
    pub ns: String,
    #[serde(rename = "DB")]
    pub db: String,
    #[serde(rename = "AC")]
    pub ac: T,
    #[serde(rename = "ID")]
    pub id: String,
}

/// Simple parser for SurrealDB record ids of the form `<table>:<id>`.
///
/// Returns `Some(RecordId)` for well-formed inputs like `user:alice`, and
/// `None` for empty, missing-colon, or multi-colon inputs.
///
/// # Example
///
/// ```rust
/// assert!(morde_rs::surrealdb::string_to_record_id("user:alice").is_some());
/// assert!(morde_rs::surrealdb::string_to_record_id("").is_none());
/// ```
pub fn string_to_record_id(s: &str) -> Option<RecordId> {
    if s.is_empty() {
        return None;
    }
    if !s.contains(':') {
        return None;
    }

    let parts = s.split(':').collect::<Vec<&str>>();
    if parts.len() != 2 {
        return None;
    }
    Some(RecordId::from((parts[0], parts[1])))
}

pub fn serialize_record_id<S>(id: &RecordId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&id.key().to_string())
}