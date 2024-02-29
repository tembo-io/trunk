use crate::errors::ExtensionRegistryError;
use crate::views::user_info::UserInfo;
use actix_web::web::Data;
use rand::{distributions::Uniform, rngs::OsRng, Rng};
use reqwest::header::HeaderValue;
use sha2::Digest;
use sqlx::{Pool, Postgres};
use tracing::error;

const TOKEN_LENGTH: usize = 32;

// Generate API token
pub fn generate_token() -> (String, [u8; 32]) {
    let plaintext = generate_secure_alphanumeric_string(TOKEN_LENGTH);
    let sha256 = hash(&plaintext);
    (plaintext, sha256)
}

// Validate token exists and returns associated user ID
pub async fn validate_token(
    token: &HeaderValue,
    conn: Data<Pool<Postgres>>,
) -> Result<UserInfo, ExtensionRegistryError> {
    // Ensure header is valid UTF-8
    let token = token.to_str()?;
    check_token_input(token)?;

    let token_digest = hash(token);

    let mut tx = conn.begin().await?;
    // Check if token exists
    let token_exists = sqlx::query!(
        "SELECT *
                FROM api_tokens
                WHERE
                    token = $1",
        &token_digest,
    )
    .fetch_optional(&mut *tx)
    .await?;

    match token_exists {
        Some(_token_exists) => {
            // Check if token has an associated user ID
            let uid = sqlx::query!(
                "SELECT user_id
                FROM api_tokens
                WHERE
                    token = $1",
                &token_digest,
            )
            .fetch_one(&mut *tx)
            .await?;

            // TODO(ianstanton) we can perform this in a single query
            let uname = sqlx::query!(
                "SELECT user_name
                FROM api_tokens
                WHERE
                    token = $1",
                &token_digest,
            )
            .fetch_one(&mut *tx)
            .await?;

            let user = UserInfo {
                user_id: uid.user_id.unwrap(),
                user_name: uname.user_name,
            };
            Ok(user)
        }
        None => {
            error!("invalid token: API token does not exist");

            Err(ExtensionRegistryError::TokenError(
                "invalid token: API token does not exist".to_owned(),
            ))
        }
    }
}

fn hash<B: AsRef<[u8]>>(to_digest: B) -> [u8; 32] {
    // Safety: this expect must not fail since Sha256::digest returns a GenericArray<u8, 32>
    sha2::Sha256::digest(to_digest)
        .as_slice()
        .try_into()
        .expect("SHA-256 must be 32 bytes")
}

fn generate_secure_alphanumeric_string(len: usize) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    OsRng
        .sample_iter(Uniform::from(0..CHARS.len()))
        .map(|idx| CHARS[idx] as char)
        .take(len)
        .collect()
}

pub fn check_token_input(input: &str) -> Result<(), ExtensionRegistryError> {
    let valid = input.as_bytes().iter().all(u8::is_ascii_alphanumeric);
    match valid {
        true => Ok(()),
        false => Err(ExtensionRegistryError::TokenError(
            "API token should only contain ASCII alphanumeric characters".to_string(),
        )),
    }
}

#[test]
fn test_generate() {
    let (plain, sha256) = generate_token();
    assert_eq!(sha256, sha2::Sha256::digest(plain.as_bytes()).as_slice());
}

#[test]
fn test_check_token() {
    let invalid = ";vBAfmrAa228VPYUnpPv5NdDWFXfkyh6I;";
    let invalid_result = check_token_input(invalid);
    assert!(matches!(
        invalid_result,
        Err(ExtensionRegistryError::TokenError(_))
    ));

    let valid = "vBAfmrAa228VPYUnpPv5NdDWFXfkyh6I";
    let valid_result = check_token_input(valid);
    assert!(matches!(valid_result, Ok(())))
}

#[cfg(test)]
mod tests {

    #[test]
    fn digests_sha256() {
        // Ref on EMPTY & NIST.1: https://www.dlitz.net/crypto/shad256-test-vectors/

        assert_eq!(
            // "EMPTY" test vector
            super::hash([]),
            [
                0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
                0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
                0x78, 0x52, 0xb8, 0x55,
            ]
        );

        assert_eq!(
            // "NIST.1" test vector
            super::hash("abc"),
            [
                0xba, 0x78, 0x16, 0xbf, 0x8f, 0x1, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
                0x22, 0x23, 0xb0, 0x3, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
                0xf2, 0x0, 0x15, 0xad
            ]
        );

        assert_eq!(
            // Test case from Google Nearby (https://developers.google.com/nearby/fast-pair/specifications/appendix/testcases)
            super::hash([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]),
            [
                0xBB, 0x00, 0x0D, 0xDD, 0x92, 0xA0, 0xA2, 0xA3, 0x46, 0xF0, 0xB5, 0x31, 0xF2, 0x78,
                0xAF, 0x06, 0xE3, 0x70, 0xF8, 0x69, 0x32, 0xCC, 0xAF, 0xCC, 0xC8, 0x92, 0xD6, 0x8D,
                0x35, 0x0F, 0x80, 0xF8
            ]
        );

        assert_eq!(
            super::hash("Hello, world!"),
            [
                0x31, 0x5f, 0x5b, 0xdb, 0x76, 0xd0, 0x78, 0xc4, 0x3b, 0x8a, 0xc0, 0x6, 0x4e, 0x4a,
                0x1, 0x64, 0x61, 0x2b, 0x1f, 0xce, 0x77, 0xc8, 0x69, 0x34, 0x5b, 0xfc, 0x94, 0xc7,
                0x58, 0x94, 0xed, 0xd3
            ]
        );
    }
}
