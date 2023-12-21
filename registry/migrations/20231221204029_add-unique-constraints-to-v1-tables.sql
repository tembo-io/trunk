ALTER TABLE v1.extension_dependency
ADD CONSTRAINT unique_dependency UNIQUE (extension_version_id, depends_on_extension_name);

ALTER TABLE v1.extension_configurations
ADD CONSTRAINT unique_extension_config UNIQUE (extension_version_id, configuration_name);

ALTER TABLE v1.trunk_project_postgres_support
ADD CONSTRAINT unique_trunk_project_postgres_support UNIQUE (postgres_version_id, trunk_project_version_id);

ALTER TABLE v1.extensions_loadable_libraries
ADD CONSTRAINT unique_extension_library UNIQUE (extension_version_id, library_name);

ALTER TABLE v1.control_file
ADD CONSTRAINT control_file_extension_version_id_unique UNIQUE (extension_version_id);
