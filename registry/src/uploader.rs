use crate::errors::ExtensionRegistryError;
use crate::views::extension_publish::ExtensionUpload;
use aws_sdk_s3;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ServerSideEncryption::Aes256;
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
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    let obj = s3_client
        .put_object()
        .bucket(bucket_name)
        .content_type(content_type)
        .body(content)
        .key(path)
        .cache_control(CACHE_CONTROL_IMMUTABLE)
        .set_server_side_encryption(Some(Aes256))
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
) -> Result<String, ExtensionRegistryError> {
    let path = extension_path(&extension.name, &extension_version.to_string(), pg_version);
    info!("Uploading extension: {:?}", extension);
    upload(bucket_name, s3_client, &path, file, "application/gzip").await?;

    Ok(path)
}
