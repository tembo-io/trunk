-- Add the library for pg_search
WITH trunk_project_cte AS (
    SELECT id
    FROM v1.trunk_project_versions
    WHERE trunk_project_name = 'pg_search'
    AND version = '0.7.6'
),
extension_version_cte AS (
    SELECT ev.id
    FROM v1.extension_versions ev
    JOIN trunk_project_cte tpv ON ev.trunk_project_version_id = tpv.id
    WHERE ev.extension_name = 'pg_search'
)
INSERT INTO v1.extensions_loadable_libraries (
    extension_version_id, library_name, requires_restart, priority
)
SELECT ev.id, 'pg_search', true, 2147483647
FROM extension_version_cte ev
ON CONFLICT (extension_version_id, library_name) 
DO UPDATE SET 
    requires_restart = EXCLUDED.requires_restart,
    priority = EXCLUDED.priority;

-- Add the library for vectorize
WITH trunk_project_cte AS (
    SELECT id
    FROM v1.trunk_project_versions
    WHERE trunk_project_name = 'vectorize'
    AND version = '0.17.0'
),
extension_version_cte AS (
    SELECT ev.id
    FROM v1.extension_versions ev
    JOIN trunk_project_cte tpv ON ev.trunk_project_version_id = tpv.id
    WHERE ev.extension_name = 'vectorize'
)
INSERT INTO v1.extensions_loadable_libraries (
    extension_version_id, library_name, requires_restart, priority
)
SELECT ev.id, 'vectorize', true, 2147483647
FROM extension_version_cte ev
ON CONFLICT (extension_version_id, library_name) 
DO UPDATE SET 
    requires_restart = EXCLUDED.requires_restart,
    priority = EXCLUDED.priority;
