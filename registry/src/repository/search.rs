use std::ops::Not;

use crate::routes::forms::SearchResult;

use super::Registry;

impl Registry {
    /// Remove an entry from the `versions` table given the related extension ID
    pub async fn search_for_terms<S: AsRef<str>>(&self, terms: &[S]) -> Result<Vec<SearchResult>, sqlx::Error> {
        let mut search_results = Vec::with_capacity(5);

        for term in terms {
            let row = sqlx::query!(
                "SELECT name, description FROM extensions
                 WHERE name ILIKE '%' || $1 || '%'
                 OR description ILIKE '%' || $1 || '%'",
                 term.as_ref()
            ).fetch_all(&self.pool)
            .await?;

            if row.is_empty().not() {
                search_results.extend(row.into_iter().map(|row| {
                    SearchResult {
                        name: row.name.unwrap(),
                        description: row.description.unwrap(),
                    }
                }))
            }
        }

        Ok(search_results)
    }
}