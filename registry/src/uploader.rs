use crate::errors::ExtensionRegistryError;
use crate::views::extension_publish::ExtensionUpload;
use aws_sdk_s3;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ServerSideEncryption::Aes256;
use base64::prelude::{Engine as _, BASE64_STANDARD};
use tracing::{debug, info};

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cache-Control
const CACHE_CONTROL_IMMUTABLE: &str = "public,max-age=31536000,immutable";

/// Returns the internal path of an uploaded extension's version archive.
fn extension_path(name: &str, extension_version: &str, pg_version: u8) -> String {
    format!("extensions/{name}/{name}-pg{pg_version}-{extension_version}.tar.gz")
}

/// Returns the URL of an uploaded extension's version archive.
///
/// The function doesn't check for the existence of the file.
pub fn extension_location(
    bucket_name: &str,
    project_name: &str,
    extension_version: &str,
) -> String {
    // Note(vini): the current download endpoint only supports Postgres 15
    let pg_version = 15;

    let host = format!("{bucket_name}.s3.amazonaws.com");
    let path = extension_path(project_name, extension_version, pg_version);
    format!("https://{host}/{path}")
}

pub async fn upload(
    bucket_name: &str,
    s3_client: &aws_sdk_s3::Client,
    path: &str,
    content: ByteStream,
    content_type: &str,
    sha256: &[u8],
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    let obj = s3_client
        .put_object()
        .bucket(bucket_name)
        .content_type(content_type)
        .body(content)
        .key(path)
        .cache_control(CACHE_CONTROL_IMMUTABLE)
        .set_server_side_encryption(Some(Aes256))
        .checksum_sha256(BASE64_STANDARD.encode(sha256))
        .send()
        .await;
    debug!("OBJECT: {:?}", obj);
    obj
}

/// Uploads an extension file.
///
/// Returns the path of the uploaded archive.
pub async fn upload_extension(
    bucket_name: &str,
    s3_client: &aws_sdk_s3::Client,
    file: ByteStream,
    extension: &ExtensionUpload,
    extension_version: &semver::Version,
    pg_version: u8,
    sha256: &[u8],
) -> Result<String, ExtensionRegistryError> {
    let path_in_bucket =
        extension_path(&extension.name, &extension_version.to_string(), pg_version);
    info!("Uploading extension: {:?}", extension);
    upload(
        bucket_name,
        s3_client,
        &path_in_bucket,
        file,
        "application/gzip",
        sha256,
    )
    .await?;

    let http_path = format!("https://{bucket_name}.s3.amazonaws.com/{path_in_bucket}");

    Ok(http_path)
}

#[cfg(test)]
mod test {
    use super::*;
    use aws_config::BehaviorVersion;
    use aws_sdk_s3 as s3;
    use aws_smithy_runtime::client::http::test_util::capture_request;
    use bytes::Bytes;
    use sha2::{Digest, Sha256};

    #[tokio::test]
    async fn test_upload() {
        // https://docs.rs/aws-smithy-runtime/1.1.3/aws_smithy_runtime/client/http/test_util/fn.capture_request.html
        let (capture_client, request) = capture_request(None);
        let client: s3::Client = s3::Client::from_conf(
            s3::Config::builder()
                .behavior_version(BehaviorVersion::latest())
                .credentials_provider(make_s3_test_credentials())
                .region(s3::config::Region::new("us-east-1"))
                .http_client(capture_client.clone())
                .build(),
        );
        let content = b"stuff";
        let file = ByteStream::from(Bytes::from_static(content));
        let sha = b"CHECKSUM";
        let res = upload(
            "test-bucket",
            &client,
            "trunk/example.txt",
            file,
            "text/plain",
            sha,
        )
        .await;
        assert!(res.is_ok());

        // Test the request.
        let req = request.expect_request();

        // Check the headers we explicitly pass in.
        assert_eq!(req.headers().get("Content-Type"), Some("text/plain"));
        assert_eq!(
            req.headers().get("x-amz-checksum-sha256"),
            Some(BASE64_STANDARD.encode(sha).as_str())
        );
        // Turns out that the SDK *also* SHA256-hashes our content. Check it.
        let mut hasher = Sha256::new();
        hasher.update(content);
        assert_eq!(
            req.headers().get("x-amz-content-sha256"),
            Some(hex::encode(hasher.finalize()).as_str()),
        );

        // Check headers generated by the AWS SDK.
        assert_eq!(
            req.headers().get("x-amz-security-token"),
            Some("trunk_session_token")
        );
        assert!(req
            .headers()
            .get("Authorization")
            .unwrap()
            .contains("TRUNK_TEST_CLIENT"));

        // Check the body.
        assert_eq!(req.body().bytes(), Some(&content[..]));
    }

    fn make_s3_test_credentials() -> s3::config::Credentials {
        s3::config::Credentials::new(
            "TRUNK_TEST_CLIENT",
            "trunk_access_key",
            Some("trunk_session_token".to_string()),
            None,
            "",
        )
    }
}
