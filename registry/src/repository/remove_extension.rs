use super::Repository;

impl Repository {
    /// Remove an entry from the `versions` table given the related extension ID
    pub async fn drop_extension_version(&self, extension_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM versions
             WHERE extension_id = $1",
            extension_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Remove an entry from the `extensions` table given its ID
    pub async fn drop_extension(&self, extension_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM extensions
             WHERE id = $1",
            extension_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Remove an entry from the `extension_owners` table given the related extension ID
    pub async fn drop_extension_owner(&self, extension_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM extension_owners
             WHERE extension_id = $1",
            extension_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Remove an entry from the `extension_owners` table given the related extension ID
    pub async fn drop_extension_category(&self, extension_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM extensions_categories
                 WHERE extension_id = $1",
            extension_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
