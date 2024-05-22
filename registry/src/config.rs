use actix_web::http::header::HeaderValue;
use std::{env, str::FromStr};

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub bucket_name: String,
    pub region: Option<String>,
    pub aws_access_key: String,
    pub aws_secret_key: String,
    pub auth_token: HeaderValue,
    pub clerk_secret_key: String,
    pub github_token: String,
    pub environment: Env,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: from_env_default(
                "DATABASE_URL",
                "postgres://postgres@localhost/trunk_registry",
            ),
            bucket_name: from_env_default("S3_BUCKET", "trunk-registry"),
            region: Some(from_env_default("S3_REGION", "")),
            aws_access_key: from_env_default("AWS_ACCESS_KEY", ""),
            aws_secret_key: from_env_default("AWS_SECRET_KEY", ""),
            auth_token: from_env_default("AUTH_TOKEN", "").parse().unwrap(),
            clerk_secret_key: env::var("CLERK_SECRET_KEY").expect("CLERK_SECRET_KEY not set"),
            github_token: env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set"),
            environment: from_env_default("ENV", "dev").parse().unwrap()
        }
    }
}

/// source a variable from environment - use default if not exists
fn from_env_default(key: &str, default: &str) -> String {
    dotenv::var(key).unwrap_or_else(|_| env::var(key).unwrap_or_else(|_| default.to_owned()))
}

#[derive(Debug, Clone)]
pub enum Env {
    Prod,
    Staging,
    Dev
}

impl FromStr for Env {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("prod") {
            Ok(Env::Prod)
        } else if s.eq_ignore_ascii_case("staging") {
            Ok(Env::Staging)
        } else if s.eq_ignore_ascii_case("dev") {
            Ok(Env::Dev)
        } else {
            Err("Invalid value: expected 'prod', 'staging' or 'dev'")
        }
    }
}