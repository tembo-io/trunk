use super::Registry;
use crate::errors::Result;

impl Registry {
    pub async fn get_extension_readme(&self, extension_name: &str) -> Result<String> {
        let record = sqlx::query!(
            "SELECT r.readme_html 
            FROM readmes AS r 
            JOIN extensions AS e ON r.extension_id = e.id 
            WHERE e.name = $1",
            extension_name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.readme_html)
    }

    /// Fetch the repository of the extension with the given name
    pub async fn get_repository_url(&self, extension_name: &str) -> Result<(i64, Option<String>)> {
        let record = sqlx::query!(
            "SELECT id, repository FROM extensions WHERE extensions.name = $1",
            extension_name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((record.id, record.repository))
    }

    pub async fn upsert_readme(&self, extension_id: i64, readme_html: &str) -> Result {
        sqlx::query!(
            "INSERT INTO readmes (extension_id, readme_html)
            VALUES ($1, $2)
            ON CONFLICT (extension_id)
            DO UPDATE SET readme_html = excluded.readme_html",
            extension_id as i32,
            readme_html
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
