use crate::errors::ExtensionRegistryError;
use actix_web::web::Data;
use log::error;
use rand::{distributions::Uniform, rngs::OsRng, Rng};
use reqwest::header::HeaderValue;
use sha2::Digest;
use sqlx::{Pool, Postgres};

const TOKEN_LENGTH: usize = 32;

// Generate API token
pub fn generate_token() -> (String, Vec<u8>) {
    let plaintext = generate_secure_alphanumeric_string(TOKEN_LENGTH);
    let sha256 = hash(&plaintext);
    (plaintext, sha256)
}

// Validate token exists and returns associated user ID
pub async fn validate_token(
    token: &HeaderValue,
    conn: Data<Pool<Postgres>>,
) -> Result<String, ExtensionRegistryError> {
    let mut tx = conn.begin().await?;
    // Check if token exists
    let token_exists = sqlx::query!(
        "SELECT *
                FROM api_tokens
                WHERE
                    token = $1",
        hash(token.to_str()?),
    )
    .fetch_optional(&mut tx)
    .await?;
    match token_exists {
        Some(_token_exists) => {
            // Check if token has an associated user ID
            let user = sqlx::query!(
                "SELECT user_id
                FROM api_tokens
                WHERE
                    token = $1",
                hash(token.to_str()?),
            )
            .fetch_one(&mut tx)
            .await?;
            Ok(user.user_id.unwrap())
        }
        None => {
            error!("invalid token: API token does not exist");
            Err(ExtensionRegistryError::TokenError(
                "invalid token: API token does not exist".to_owned(),
            ))
        }
    }
}

fn hash(plaintext: &str) -> Vec<u8> {
    sha2::Sha256::digest(plaintext.as_bytes())
        .as_slice()
        .to_vec()
}

fn generate_secure_alphanumeric_string(len: usize) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    OsRng
        .sample_iter(Uniform::from(0..CHARS.len()))
        .map(|idx| CHARS[idx] as char)
        .take(len)
        .collect()
}

#[test]
fn test_generate() {
    let (plain, sha256) = generate_token();
    assert_eq!(sha256, sha2::Sha256::digest(plain.as_bytes()).as_slice());
}
