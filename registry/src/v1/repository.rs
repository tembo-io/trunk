use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

use crate::errors::Result;
use crate::repository::Registry;
use crate::views::extension_publish::{
    ControlFileMetadata, ExtensionConfiguration, LoadableLibrary,
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct TrunkProjectView {
    pub name: String,
    pub description: String,
    pub documentation_link: Option<String>,
    pub repository_link: String,
    pub version: String,
    pub postgres_versions: Option<Vec<u8>>,
    pub extensions: Vec<ExtensionView>,
    pub downloads: Option<Vec<Download>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Download {
    pub link: String,
    pub pg_version: u8,
    pub architecture: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExtensionView {
    pub extension_name: String,
    pub version: String,
    pub trunk_project_name: String,
    pub dependencies_extension_names: Option<Vec<String>>,
    pub loadable_libraries: Option<Vec<LoadableLibrary>>,
    pub configurations: Option<Vec<ExtensionConfiguration>>,
    pub control_file: Option<ControlFileMetadata>,
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
                    'postgres_versions', (
                        SELECT json_agg(pg.major)
                        FROM v1.trunk_project_postgres_support tpps
                        JOIN v1.postgres_version pg ON tpps.postgres_version_id = pg.id
                        WHERE tpps.trunk_project_version_id = latest_tpvs.id
                    ),
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
                                    'default', ec.recommended_default_value
                                ))
                                FROM v1.extension_configurations ec
                                WHERE ec.extension_version_id = ev.id
                            ),
                            'control_file', (
                                SELECT json_build_object(
                                    'absent', cf.absent,
                                    'content', cf.content
                                )
                                FROM v1.control_file cf
                                WHERE cf.extension_version_id = ev.id
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
                    'postgres_versions', (
                        SELECT json_agg(pg.major)
                        FROM v1.trunk_project_postgres_support tpps
                        JOIN v1.postgres_version pg ON tpps.postgres_version_id = pg.id
                        WHERE tpps.trunk_project_version_id = tpv.id
                    ),
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
                                    'default', ec.recommended_default_value
                                ))
                                FROM v1.extension_configurations ec
                                WHERE ec.extension_version_id = ev.id
                            ),
                            'control_file', (
                                SELECT json_build_object(
                                    'absent', cf.absent,
                                    'content', cf.content
                                )
                                FROM v1.control_file cf
                                WHERE cf.extension_version_id = ev.id
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
                    'postgres_versions', (
                        SELECT json_agg(pg.major)
                        FROM v1.trunk_project_postgres_support tpps
                        JOIN v1.postgres_version pg ON tpps.postgres_version_id = pg.id
                        WHERE tpps.trunk_project_version_id = tpv.id
                    ),
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
                                    'default', ec.recommended_default_value
                                ))
                                FROM v1.extension_configurations ec
                                WHERE ec.extension_version_id = ev.id
                            ),
                            'control_file', (
                                SELECT json_build_object(
                                    'absent', cf.absent,
                                    'content', cf.content
                                )
                                FROM v1.control_file cf
                                WHERE cf.extension_version_id = ev.id
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
                'postgres_versions', (
                    SELECT json_agg(pg.major)
                    FROM v1.trunk_project_postgres_support tpps
                    JOIN v1.postgres_version pg ON tpps.postgres_version_id = pg.id
                    WHERE tpps.trunk_project_version_id = tpv.id
                ),
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
                                'default', ec.recommended_default_value
                            ))
                            FROM v1.extension_configurations ec
                            WHERE ec.extension_version_id = ev.id
                        ),
                        'control_file', (
                            SELECT json_build_object(
                                'absent', cf.absent,
                                'content', cf.content
                            )
                            FROM v1.control_file cf
                            WHERE cf.extension_version_id = ev.id
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
        tracing::info!("Inserted Trunk project name");

        // 2. insert trunk project version
        // Note: UNIQUE constraint will avoid re-inserting a previously existing tuple of Trunk name and version
        let record = sqlx::query!(
            "INSERT INTO v1.trunk_project_versions (trunk_project_name, version, description, repository_link, documentation_link)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (trunk_project_name, version) 
            DO UPDATE SET 
                description = EXCLUDED.description,
                repository_link = EXCLUDED.repository_link,
                documentation_link = EXCLUDED.documentation_link,
                -- Dummy update to ensure a row is always returned
                id = v1.trunk_project_versions.id
            RETURNING id",
            trunk_project.name, trunk_project.version, trunk_project.description, trunk_project.repository_link, trunk_project.documentation_link
        ).fetch_one(&self.pool).await?;
        tracing::info!("Inserted Trunk project version");

        let trunk_project_version_id = record.id;

        // 3. insert Postgres version and download URLs
        tracing::info!("Inserting Postgres version!");
        if let Some(supported_postgres_versions) = trunk_project.postgres_versions {
            let downloads = trunk_project.downloads.as_deref().unwrap_or_default();
            for pg_version in supported_postgres_versions {
                sqlx::query!(
                    "INSERT INTO v1.trunk_project_postgres_support(postgres_version_id, trunk_project_version_id)
                    SELECT pg.id, $1
                    FROM v1.postgres_version pg
                    WHERE pg.major = $2",
                    trunk_project_version_id,
                    pg_version as i32
                ).execute(&self.pool).await?;

                let maybe_download = downloads
                    .iter()
                    .find(|download| download.pg_version == pg_version);

                if let Some(download) = maybe_download {
                    self.insert_download_link(
                        pg_version,
                        trunk_project_version_id,
                        &download.link,
                        &download.sha256,
                    )
                    .await?;
                }
            }
        }

        for extension in &trunk_project.extensions {
            // 4. insert extension version (or versions)
            let record = sqlx::query!(
                "INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
                VALUES ($1, $2, $3)
                ON CONFLICT (extension_name, trunk_project_version_id, version) 
                DO UPDATE SET
                    -- Dummy update to ensure a row is always returned
                    id = v1.extension_versions.id
                RETURNING id",
                extension.extension_name,
                trunk_project_version_id,
                extension.version
            ).fetch_one(&self.pool).await?;
            let extension_version_id = record.id;
            tracing::info!("Inserted extension version");

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

            // 5. insert extension dependencies
            for dependency_name in dependencies {
                sqlx::query!(
                    "INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
                    VALUES ($1, $2)
                    ON CONFLICT (extension_version_id, depends_on_extension_name) 
                    DO NOTHING",
                    extension_version_id,
                    dependency_name,
                ).execute(&self.pool).await?;
            }

            // 6. insert extension configurations
            self.insert_configurations(extension_version_id, configurations)
                .await?;

            // 7. insert shared preload libraries
            self.insert_loadable_libraries(extension_version_id, loadable_libraries)
                .await?;

            // 8. Insert control file metadata
            if let Some(control_file) = &extension.control_file {
                self.insert_control_file(extension_version_id, control_file.clone())
                    .await?;
            }
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
                    VALUES ($1, $2, $3, $4)
                    ON CONFLICT (extension_version_id, configuration_name)
                    DO UPDATE SET 
                        is_required = EXCLUDED.is_required,
                        recommended_default_value = EXCLUDED.recommended_default_value",
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
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (extension_version_id, library_name)
                DO UPDATE SET 
                    requires_restart = EXCLUDED.requires_restart,
                    priority = EXCLUDED.priority",
                extension_version_id,
                library.library_name,
                library.requires_restart,
                library.priority,
            ).execute(&self.pool).await?;
        }

        Ok(())
    }

    async fn insert_control_file(
        &self,
        extension_version_id: i32,
        control_file: ControlFileMetadata,
    ) -> Result {
        sqlx::query!(
            "INSERT INTO v1.control_file (extension_version_id, absent, content)
            VALUES ($1, $2, $3)
            ON CONFLICT (extension_version_id) 
            DO UPDATE SET 
                absent = EXCLUDED.absent,
                content = EXCLUDED.content",
            extension_version_id,
            control_file.absent,
            control_file.content,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn insert_download_link(
        &self,
        postgres_major: u8,
        trunk_project_version_id: i32,
        download_url: &str,
        sha256: &str,
    ) -> Result {
        sqlx::query!(
            "INSERT INTO v1.trunk_project_downloads (
                platform_id,
                postgres_version_id,
                trunk_project_version_id,
                download_url,
                sha256
            ) VALUES (
                -- TODO: this works because this is currently the only platform supported, must change
                -- if more get supported
                (SELECT id FROM v1.platform WHERE platform_name = 'linux/amd64'),
                (SELECT id FROM v1.postgres_version WHERE major = $1),
                $2,
                $3,
                $4
            )
            ON CONFLICT (platform_id, postgres_version_id, trunk_project_version_id, download_url)
            DO UPDATE SET
                sha256 = EXCLUDED.sha256,
                download_url = EXCLUDED.download_url",
            postgres_major as i32,
            trunk_project_version_id,
            download_url,
            sha256
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
