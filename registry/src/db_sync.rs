use actix_web::rt::time as tokio_time;
use anyhow::Context;
use sqlx::PgPool;
use std::{env, time::Duration};

use crate::config::Env;

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

    sqlx::query(&query)
        .execute(conn)
        .await?;

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

    sqlx::query(&query)
        .execute(conn)
        .await?;

    Ok(())
}

async fn create_clone_function(conn: &PgPool, env: Env) -> anyhow::Result<()> {
    let s3_prefix = match env {
        Env::Prod => unreachable!(),
        Env::Staging => "use1-staging-pgtrunkio",
        Env::Dev => "use1-dev-pgtrunkio",
    };

    // Create the clone function with dynamic s3_prefix
    let query = format!(
        "
        CREATE OR REPLACE FUNCTION clone_prod_trunk() RETURNS void LANGUAGE plpgsql AS $$
        DECLARE
            table_rec RECORD; 
        BEGIN
            EXECUTE 'DROP SCHEMA IF EXISTS foreign_v1 CASCADE';
            EXECUTE 'CREATE SCHEMA foreign_v1';
            EXECUTE 'IMPORT FOREIGN SCHEMA \"v1\" FROM SERVER prod_trunk INTO foreign_v1';
            EXECUTE 'DROP SCHEMA IF EXISTS v1 CASCADE';
            EXECUTE 'CREATE SCHEMA v1';

            FOR table_rec IN SELECT table_name FROM information_schema.tables WHERE table_schema = 'foreign_v1' AND table_type = 'FOREIGN'
            LOOP
                EXECUTE format('CREATE TABLE v1.%I AS SELECT * FROM foreign_v1.%I', table_rec.table_name, table_rec.table_name);
            END LOOP;

            EXECUTE 'UPDATE v1.trunk_project_downloads SET download_url = REPLACE(download_url, ''use1-prod-pgtrunkio'', ''{s3_prefix}'') WHERE download_url LIKE ''%use1-prod-pgtrunkio%''';
        END $$;
        "
    );

    sqlx::query(&query).execute(conn).await?;

    Ok(())
}

pub async fn sync_trunk_db_and_s3(conn: PgPool, env: Env) {
    let fdw = match ForeignDataWrapper::new() {
        Ok(fdw) => fdw,
        Err(err) => {
            tracing::error!(
                "db_sync: Failed to get Foreign Data Wrapper arguments: {err}. Exiting."
            );
            return;
        }
    };

    if let Err(err) = create_trunk_fdw(&conn, &fdw).await {
        tracing::error!("Failed to create Foreign Data Wrapper: {err}");
        return;
    }

    if let Err(err) = create_mapping(&conn, &fdw).await {
        tracing::error!("Failed to create mapping: {err}");
        return;
    }

    if let Err(err) = create_clone_function(&conn, env).await {
        tracing::error!("Failed to create `clone_prod_trunk`: {err}");
        return;
    }

    loop {
        if let Err(err) = sqlx::query("SELECT clone_prod_trunk()")
            .execute(&conn)
            .await {
                tracing::error!("Failed to SELECT clone_prod_trunk: {err}");
            }

        tracing::info!("Cloned prod Trunk successfully!");

        // TODO: save when the last sync was done in the DB
        // and then sleep according?
        tokio_time::sleep(Duration::from_secs(60 * 60)).await;        
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
