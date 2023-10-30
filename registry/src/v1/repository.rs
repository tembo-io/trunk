use serde::{Deserialize, Serialize};

use crate::errors::Result;
use crate::repository::Registry;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrunkProjectView {
    pub name: String,
    pub description: String,
    pub documentation_link: String,
    pub repository_link: String,
    pub version: String,
    pub extensions: Vec<ExtensionView>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionConfigurationView {
    pub name: String,
    pub is_required: bool,
    pub recommended_default: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionView {
    pub extension_name: String,
    pub version: String,
    pub trunk_project_name: String,
    pub dependencies_extension_names: Option<Vec<String>>,
    pub requires_restart: bool,
    pub configurations: Option<Vec<ExtensionConfigurationView>>,
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
                            'requires_restart', (
                                SELECT CASE
                                    WHEN count(*) > 0 THEN true
                                    ELSE false
                                END
                                FROM v1.extensions_loadable_libraries ell
                                WHERE ell.extension_version_id = ev.id
                                AND requires_restart = true
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
}
