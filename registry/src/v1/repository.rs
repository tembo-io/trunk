use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

use crate::errors::Result;
use crate::repository::Registry;
use crate::views::extension_publish::{ExtensionConfiguration, LoadableLibrary};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct TrunkProjectView {
    pub name: String,
    pub description: String,
    pub documentation_link: Option<String>,
    pub repository_link: String,
    pub version: String,
    pub extensions: Vec<ExtensionView>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExtensionView {
    pub extension_name: String,
    pub version: String,
    pub trunk_project_name: String,
    pub dependencies_extension_names: Option<Vec<String>>,
    pub loadable_libraries: Option<Vec<LoadableLibrary>>,
    pub configurations: Option<Vec<ExtensionConfiguration>>,
}

impl Registry {
    pub async fn all_trunk_projects(&self) -> Result<Vec<TrunkProjectView>> {
        let records = sqlx::query!("
            SELECT
                json_build_object(
                    'name', tp.name,
                    'description', latest_tpvs.description,
                    'documentation_link', latest_tpvs.documentation_link,
                    'repository_link', latest_tpvs.repository_link,
                    'version', latest_tpvs.version,
                    'extensions', (
                        SELECT json_agg(json_build_object(
                            'extension_name', ev.extension_name,
                            'version', ev.version,
                            'trunk_project_name', tp.name,
                            'dependencies_extension_names', (
                                SELECT json_agg(ed.depends_on_extension_name)
                                FROM v1.extension_dependency ed
                                WHERE ed.extension_version_id = ev.id
                            ),
                            'loadable_libraries', (
                                SELECT json_agg(json_build_object(
                                    'library_name', ell.library_name,
                                    'requires_restart', ell.requires_restart,
                                    'priority', ell.priority
                                ))
                                FROM v1.extensions_loadable_libraries ell
                                WHERE ell.extension_version_id = ev.id
                            ),
                            'configurations', (
                                SELECT json_agg(json_build_object(
                                    'name', ec.configuration_name,
                                    'is_required', ec.is_required,
                                    'recommended_default', ec.recommended_default_value
                                ))
                                FROM v1.extension_configurations ec
                                WHERE ec.extension_version_id = ev.id
                            )
                        ))
                        FROM v1.extension_versions ev
                        WHERE ev.trunk_project_version_id = latest_tpvs.id
                    )
                ) AS result
            FROM
                v1.trunk_project tp
            JOIN (
                SELECT tpv.*
                FROM v1.trunk_project_versions tpv
                JOIN (
                        SELECT trunk_project_name, MAX(string_to_array(version, '.')::int[]) as max_version
                        FROM v1.trunk_project_versions
                        GROUP BY trunk_project_name
                    ) sub_tpv
                ON tpv.trunk_project_name = sub_tpv.trunk_project_name
                AND string_to_array(tpv.version, '.')::int[] = sub_tpv.max_version
                ) latest_tpvs
            ON tp.name = latest_tpvs.trunk_project_name
            ORDER BY tp.name"
        ).fetch_all(&self.pool).await?;

        Ok(records
            .into_iter()
            .flat_map(|record| record.result)
            .flat_map(serde_json::from_value)
            .collect())
    }

    pub async fn trunk_projects_by_extension_name(
        &self,
        extension_name: &str,
    ) -> Result<Vec<TrunkProjectView>> {
        let records = sqlx::query!(
            "SELECT
                json_build_object(
                    'name', tpv.trunk_project_name,
                    'description', tpv.description,
                    'documentation_link', tpv.documentation_link,
                    'repository_link', tpv.repository_link,
                    'version', tpv.version,
                    'extensions', (
                        SELECT json_agg(json_build_object(
                            'extension_name', ev.extension_name,
                            'version', ev.version,
                            'trunk_project_name', tpv.trunk_project_name,
                            'dependencies_extension_names', (
                                SELECT json_agg(ed.depends_on_extension_name)
                                FROM v1.extension_dependency ed
                                WHERE ed.extension_version_id = ev.id
                            ),
                            'loadable_libraries', (
                                SELECT json_agg(json_build_object(
                                    'library_name', ell.library_name,
                                    'requires_restart', ell.requires_restart,
                                    'priority', ell.priority
                                ))
                                FROM v1.extensions_loadable_libraries ell
                                WHERE ell.extension_version_id = ev.id
                            ),
                            'configurations', (
                                SELECT json_agg(json_build_object(
                                    'name', ec.configuration_name,
                                    'is_required', ec.is_required,
                                    'recommended_default', ec.recommended_default_value
                                ))
                                FROM v1.extension_configurations ec
                                WHERE ec.extension_version_id = ev.id
                            )
                        ))
                        FROM v1.extension_versions ev
                        WHERE ev.trunk_project_version_id = tpv.id
                    )
                ) AS result
            FROM
                v1.trunk_project_versions tpv
            JOIN v1.extension_versions ev ON ev.trunk_project_version_id = tpv.id
            JOIN (
                SELECT extension_name, MAX(string_to_array(version, '.')::int[]) as max_version
                FROM v1.extension_versions
                WHERE extension_name = $1
                GROUP BY extension_name
            ) sub_ev ON ev.extension_name = sub_ev.extension_name AND string_to_array(ev.version, '.')::int[] = sub_ev.max_version
            ", extension_name
        ).fetch_all(&self.pool).await?;

        Ok(records
            .into_iter()
            .flat_map(|record| record.result)
            .flat_map(serde_json::from_value)
            .collect())
    }

    pub async fn trunk_projects_by_name(
        &self,
        trunk_project_name: &str,
    ) -> Result<Vec<TrunkProjectView>> {
        let records = sqlx::query!(
            "SELECT
                json_build_object(
                    'name', tpv.trunk_project_name,
                    'description', tpv.description,
                    'version', tpv.version,
                    'documentation_link', tpv.documentation_link,
                    'repository_link', tpv.repository_link,
                    'extensions', (
                        SELECT json_agg(json_build_object(
                            'extension_name', ev.extension_name,
                            'version', ev.version,
                            'trunk_project_name', tpv.trunk_project_name,
                            'dependencies_extension_names', (
                                SELECT json_agg(ed.depends_on_extension_name)
                                FROM v1.extension_dependency ed
                                WHERE ed.extension_version_id = ev.id
                            ),
                            'loadable_libraries', (
                                SELECT json_agg(json_build_object(
                                    'library_name', ell.library_name,
                                    'requires_restart', ell.requires_restart,
                                    'priority', ell.priority
                                ))
                                FROM v1.extensions_loadable_libraries ell
                                WHERE ell.extension_version_id = ev.id
                            ),
                            'configurations', (
                                SELECT json_agg(json_build_object(
                                    'name', ec.configuration_name,
                                    'is_required', ec.is_required,
                                    'recommended_default', ec.recommended_default_value
                                ))
                                FROM v1.extension_configurations ec
                                WHERE ec.extension_version_id = ev.id
                            )
                        ))
                        FROM v1.extension_versions ev
                        WHERE ev.trunk_project_version_id = tpv.id
                    )
                ) AS result
            FROM
                v1.trunk_project_versions tpv
            WHERE
                tpv.trunk_project_name = $1",
            trunk_project_name
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .flat_map(|record| record.result)
            .flat_map(serde_json::from_value)
            .collect())
    }

    pub async fn trunk_projects_by_name_and_version(
        &self,
        trunk_project_name: &str,
        trunk_project_version: &str,
    ) -> Result<Vec<TrunkProjectView>> {
        let records = sqlx::query!(
            "SELECT json_build_object(
                'name', tpv.trunk_project_name,
                'description', tpv.description,
                'version', tpv.version,
                'documentation_link', tpv.documentation_link,
                'repository_link', tpv.repository_link,
                'extensions', (
                    SELECT json_agg(json_build_object(
                        'extension_name', ev.extension_name,
                        'version', ev.version,
                        'trunk_project_name', tpv.trunk_project_name,
                        'dependencies_extension_names', (
                            SELECT json_agg(ed.depends_on_extension_name)
                            FROM v1.extension_dependency ed
                            WHERE ed.extension_version_id = ev.id
                        ),
                        'loadable_libraries', (
                            SELECT json_agg(json_build_object(
                                'library_name', ell.library_name,
                                'requires_restart', ell.requires_restart,
                                'priority', ell.priority
                            ))
                            FROM v1.extensions_loadable_libraries ell
                            WHERE ell.extension_version_id = ev.id
                        ),
                        'configurations', (
                            SELECT json_agg(json_build_object(
                                'name', ec.configuration_name,
                                'is_required', ec.is_required,
                                'recommended_default', ec.recommended_default_value
                            ))
                            FROM v1.extension_configurations ec
                            WHERE ec.extension_version_id = ev.id
                        )
                    ))
                    FROM v1.extension_versions ev
                    WHERE ev.trunk_project_version_id = tpv.id
                )
            ) AS result
        FROM
            v1.trunk_project_versions tpv
        WHERE
            tpv.trunk_project_name = $1
            AND tpv.version = $2",
            trunk_project_name,
            trunk_project_version
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .flat_map(|record| record.result)
            .flat_map(serde_json::from_value)
            .collect())
    }

    /// Insert a Trunk project (and related information) into the database.
    /// Follows the given steps:
    ///   1. insert trunk project name
    ///   2. insert trunk project version
    ///   3. insert extension version
    ///   4. insert extension dependencies
    ///   5. insert extension configurations
    ///   6. insert shared preload libraries
    pub async fn insert_trunk_project(&self, trunk_project: TrunkProjectView) -> Result<()> {
        // 1. insert trunk project name
        sqlx::query!(
            "INSERT INTO v1.trunk_project (name) 
            VALUES ($1)
            ON CONFLICT (name) DO NOTHING",
            trunk_project.name
        )
        .execute(&self.pool)
        .await?;

        // 2. insert trunk project version
        // Note: UNIQUE constraint will avoid re-inserting a previously existing tuple of Trunk name and version
        let record = sqlx::query!(
            "INSERT INTO v1.trunk_project_versions (trunk_project_name, version, description, repository_link, documentation_link)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id",
            trunk_project.name, trunk_project.version, trunk_project.description, trunk_project.repository_link, trunk_project.documentation_link
        ).fetch_one(&self.pool).await?;
        let trunk_project_version_id = record.id;

        for extension in &trunk_project.extensions {
            // 3. insert extension version (or versions)
            let record = sqlx::query!(
                "INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
                VALUES ($1, $2, $3)
                ON CONFLICT (extension_name, trunk_project_version_id, version) 
                DO NOTHING
                RETURNING id",
                extension.extension_name,
                trunk_project_version_id,
                extension.version
            ).fetch_one(&self.pool).await?;
            let extension_version_id = record.id;

            let dependencies = extension
                .dependencies_extension_names
                .iter()
                .flat_map(|deps| deps.iter());
            let configurations = extension
                .configurations
                .iter()
                .flat_map(|configs| configs.iter());
            let loadable_libraries = extension
                .loadable_libraries
                .iter()
                .flat_map(|libs| libs.iter());

            // 4. insert extension dependencies
            for dependency_name in dependencies {
                sqlx::query!(
                    "INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
                    VALUES ($1, $2)",
                    extension_version_id,
                    dependency_name,
                ).execute(&self.pool).await?;
            }

            // 5. insert extension configurations
            self.insert_configurations(extension_version_id, configurations)
                .await?;

            // 6. insert shared preload libraries
            self.insert_loadable_libraries(extension_version_id, loadable_libraries)
                .await?;
        }
        Ok(())
    }

    async fn insert_configurations(
        &self,
        extension_version_id: i32,
        configurations: impl Iterator<Item = &ExtensionConfiguration>,
    ) -> Result {
        for config in configurations {
            sqlx::query!(
                    "INSERT INTO v1.extension_configurations (extension_version_id, is_required, configuration_name, recommended_default_value)
                    VALUES ($1, $2, $3, $4)",
                    extension_version_id,
                    config.is_required,
                    config.configuration_name,
                    config.recommended_default_value,
                ).execute(&self.pool).await?;
        }
        Ok(())
    }

    async fn insert_loadable_libraries(
        &self,
        extension_version_id: i32,
        loadable_libraries: impl Iterator<Item = &LoadableLibrary>,
    ) -> Result {
        for library in loadable_libraries {
            sqlx::query!(
                "INSERT INTO v1.extensions_loadable_libraries (extension_version_id, library_name, requires_restart, priority)
                VALUES ($1, $2, $3, $4)",
                extension_version_id,
                library.library_name,
                library.requires_restart,
                library.priority,
            ).execute(&self.pool).await?;
        }

        Ok(())
    }
}
