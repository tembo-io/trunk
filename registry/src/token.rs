use rand::{distributions::Uniform, rngs::OsRng, Rng};
use sha2::{Digest};

const TOKEN_LENGTH: usize = 32;

pub fn generate() -> (String, Vec<u8>) {
    let plaintext = format!(
        "{}",
        generate_secure_alphanumeric_string(TOKEN_LENGTH)
    );
    let sha256 = hash(&plaintext);

    (plaintext, sha256)
}

fn hash(plaintext: &str) -> Vec<u8> {
    sha2::Sha256::digest(plaintext.as_bytes()).as_slice().to_vec()
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
    let (plain, sha256) = generate();
    assert_eq!(sha256, sha2::Sha256::digest(plain.as_bytes()).as_slice());
}
