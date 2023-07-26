use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::{conn_options, errors};

/// Queries related to removing an extension from the DB
mod remove_extension;

/// Abstracts over database interactions
#[derive(Clone)]
pub struct Repository {
    pool: Pool<Postgres>,
}

impl Repository {
    /// Connect to Postgres parsing URL options
    pub async fn connect(url: &str) -> Result<Self, errors::ExtensionRegistryError> {
        let options = conn_options(url)?;

        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(10))
            .max_connections(15)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    /// Get an extension's ID given its name.
    pub async fn extension_id(&self, name: &str) -> Result<Option<i64>, sqlx::Error> {
        let maybe_record = sqlx::query!("SELECT id FROM extensions WHERE name = $1", name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(maybe_record.map(|record| record.id))
    }

    pub async fn purge_extension(&self, extension_id: i64) -> Result<(), sqlx::Error> {
        // Remove the versioning information regarding this extension
        self.drop_extension_version(extension_id as i32).await?;

        // Remove the metadata on this extension
        self.drop_extension(extension_id).await?;

        // Remove the info on who owned this extension
        self.drop_extension_owner(extension_id as i32).await?;

        // Remove what categories this extension used to be in
        self.drop_extension_category(extension_id as i32).await?;

        Ok(())
    }
}
