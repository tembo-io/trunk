use super::Repository;

type CategoryId = i32;

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

    /// Remove an entry from the `extension_owners` table given the related extension ID, returning
    /// all affected categories
    pub async fn drop_extension_category(
        &self,
        extension_id: i32,
    ) -> Result<impl Iterator<Item = CategoryId>, sqlx::Error> {
        let records = sqlx::query!(
            "DELETE FROM extensions_categories
                 WHERE extension_id = $1
                 RETURNING category_id
                 ",
            extension_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records.into_iter().map(|record| record.category_id))
    }
}
