use actix_web::rt::task::spawn_blocking;
use actix_web::rt::time as tokio_time;
use anyhow::{bail, Context};
use sqlx::PgPool;
use std::{env, process::Command, time::Duration};

// TODO(vini): attempt to do this using aws_sdk_s3 instead?
async fn copy_s3_files(buckets: &S3Buckets) -> anyhow::Result<()> {
    let source_bucket = buckets.source_bucket.clone();
    let dest_bucket = buckets.destination_bucket.clone();

    // Spawn blocking since actix::rt does not reexport tokio's Command
    let handle = spawn_blocking(move || {
        // Run the aws s3 sync command as a subprocess
        Command::new("aws")
            .arg("s3")
            .arg("sync")
            .arg(source_bucket)
            .arg(dest_bucket)
            .output()
            .with_context(|| "Failed to execute aws s3 sync command")
    });

    let output = handle
        .await
        .with_context(|| "Failed to run blocking task: aws s3 sync")??;

    let stdout = String::from_utf8(output.stdout)
        .with_context(|| "AWS CLI returned invalid UTF-8 in stdout")?;
    let stderr = String::from_utf8(output.stderr)
        .with_context(|| "AWS CLI returned invalid UTF-8 in stderr")?;

    let source_bucket = &buckets.source_bucket;
    let dest_bucket = &buckets.destination_bucket;

    // Check the command output
    if output.status.success() {
        tracing::info!("Successfully synced {} to {}", source_bucket, dest_bucket);
        tracing::info!("Output: {stdout}");
    } else {
        tracing::error!(
            "Error syncing {} to {}. stdout: {stdout}\nstderr: {stderr}",
            source_bucket,
            dest_bucket
        );
        bail!("Failed to run aws s3 sync: {stderr}")
    }

    Ok(())
}

async fn create_mapping(conn: &PgPool, fdw: &ForeignDataWrapper) -> anyhow::Result<()> {
    let query = format!(
        "
        DO $$
        BEGIN
            IF NOT EXISTS (SELECT 1 FROM pg_user_mappings WHERE srvname = 'prod_trunk' AND usename = 'postgres') THEN
                EXECUTE 'CREATE USER MAPPING FOR postgres SERVER prod_trunk 
                OPTIONS (user ''{}'', password ''{}'')';
            END IF;
        END $$;
        ",
        fdw.db_user, fdw.db_password
    );

    sqlx::query(&query).execute(conn).await?;

    Ok(())
}

async fn create_trunk_fdw(conn: &PgPool, fdw: &ForeignDataWrapper) -> anyhow::Result<()> {
    sqlx::query("CREATE EXTENSION IF NOT EXISTS postgres_fdw")
        .execute(conn)
        .await?;

    let query = format!(
        "
        DO $$
        BEGIN
            IF NOT EXISTS (SELECT 1 FROM pg_foreign_server WHERE srvname = 'prod_trunk') THEN
                EXECUTE 'CREATE SERVER prod_trunk FOREIGN DATA WRAPPER postgres_fdw 
                OPTIONS (host ''{}'', port ''{}'', dbname ''{}'')';
            END IF;
        END $$;
        ",
        fdw.host, fdw.port, fdw.db_name
    );

    sqlx::query(&query).execute(conn).await?;

    Ok(())
}

async fn create_clone_function(conn: &PgPool, buckets: &S3Buckets) -> anyhow::Result<()> {
    let source_s3_prefix = &buckets.source_bucket_prefix;
    let dest_s3_prefix = &buckets.destination_bucket_prefix;

    // Create the clone function with dynamic s3_prefix
    let query = format!(
        r#"
        CREATE OR REPLACE FUNCTION clone_prod_trunk() RETURNS void LANGUAGE plpgsql AS $$
        DECLARE
            table_rec RECORD; 
        BEGIN
            DROP SCHEMA IF EXISTS foreign_v1 CASCADE;
            CREATE SCHEMA foreign_v1;
            EXECUTE 'IMPORT FOREIGN SCHEMA "v1" FROM SERVER prod_trunk INTO foreign_v1';

            DROP SCHEMA IF EXISTS v1 CASCADE;
            CREATE SCHEMA v1;

            -- Clone tables from foreign_v1 into v1
            FOR table_rec IN SELECT table_name FROM information_schema.tables WHERE table_schema = 'foreign_v1' AND table_type = 'FOREIGN'
            LOOP
                EXECUTE format('CREATE TABLE v1.%I AS SELECT * FROM foreign_v1.%I', table_rec.table_name, table_rec.table_name);
            END LOOP;

            -- Update s3 bucket download links
            UPDATE v1.trunk_project_downloads 
            SET download_url = REPLACE(download_url, '{source_s3_prefix}', '{dest_s3_prefix}')
            WHERE download_url LIKE '%use1-prod-pgtrunkio%';

            -- Clone tables from foreign public into public
            EXECUTE 'IMPORT FOREIGN SCHEMA "public" FROM SERVER prod_trunk INTO public';

        END $$;
        "#
    );

    sqlx::query(&query).execute(conn).await?;

    Ok(())
}

pub async fn sync_trunk_db_and_s3(conn: PgPool) {
    let buckets = match S3Buckets::new() {
        Ok(buckets) => buckets,
        Err(err) => {
            tracing::error!("db_sync: Missing S3 bucket env variable: {err}. Exiting.");
            return;
        }
    };

    let fdw = match ForeignDataWrapper::new() {
        Ok(fdw) => fdw,
        Err(err) => {
            tracing::error!(
                "db_sync: Failed to get Foreign Data Wrapper arguments: {err}. Exiting."
            );
            return;
        }
    };

    if let Err(err) = copy_s3_files(&buckets).await {
        tracing::error!("Failed to sync S3 buckets: {err}");
        return;
    }

    if let Err(err) = create_trunk_fdw(&conn, &fdw).await {
        tracing::error!("Failed to create Foreign Data Wrapper: {err}");
        return;
    }

    if let Err(err) = create_mapping(&conn, &fdw).await {
        tracing::error!("Failed to create mapping: {err}");
        return;
    }

    if let Err(err) = create_clone_function(&conn, &buckets).await {
        tracing::error!("Failed to create `clone_prod_trunk`: {err}");
        return;
    }

    loop {
        if let Err(err) = sqlx::query("SELECT clone_prod_trunk()")
            .execute(&conn)
            .await
        {
            tracing::error!("Failed to SELECT clone_prod_trunk: {err}");
        }

        tracing::info!("Cloned prod Trunk successfully!");

        // TODO: save when the last sync was done in the DB
        // and then sleep according?
        tokio_time::sleep(Duration::from_secs(60 * 60)).await;
    }
}

pub struct S3Buckets {
    source_bucket: String,
    destination_bucket: String,
    source_bucket_prefix: String,
    destination_bucket_prefix: String,
}

impl S3Buckets {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            source_bucket: env::var("SOURCE_BUCKET")?,
            destination_bucket: env::var("DEST_BUCKET")?,
            source_bucket_prefix: env::var("SOURCE_BUCKET_PREFIX")?,
            destination_bucket_prefix: env::var("DEST_BUCKET_PREFIX")?,
        })
    }
}

pub struct ForeignDataWrapper {
    host: String,
    port: u16,
    db_name: String,
    db_user: String,
    db_password: String,
}

impl ForeignDataWrapper {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            host: env::var("FDW_HOST")?,
            port: env::var("FDW_PORT")?
                .parse()
                .with_context(|| "Expected FDW_PORT to be an integer and fit in a u16")?,
            db_name: env::var("FDW_DBNAME").unwrap_or_else(|_| "postgres".into()),
            db_user: env::var("FDW_USER").unwrap_or_else(|_| "readonly".into()),
            db_password: env::var("FDW_PASSWORD")?,
        })
    }
}
