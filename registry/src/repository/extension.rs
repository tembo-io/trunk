use super::Registry;
use crate::errors::Result;

impl Registry {
    /// Fetch the repository of the extension with the given name
    pub async fn get_repository_url(
        &self,
        extension_name: &str,
    ) -> Result<(i64, Option<String>)> {
        let record = sqlx::query!(
            "SELECT id, repository FROM extensions WHERE extensions.name = $1",
            extension_name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((record.id, record.repository))
    }

    pub async fn save_readme(&self, extension_id: i64, readme_html: &str) -> Result {
        // TODO

        Ok(())
    }
}
