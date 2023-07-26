use super::Repository;

impl Repository {
    /// Increase the extension count for the given category
    pub async fn increase_extension_count(&self, category_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            UPDATE categories
            SET extension_count = extension_count + 1
            WHERE id = $1
            ",
            category_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Decrease the extension count for the given category
    pub async fn decrease_extension_count(&self, category_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
                UPDATE categories
                SET extension_count = extension_count - 1
                WHERE id = $1
                ",
            category_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
