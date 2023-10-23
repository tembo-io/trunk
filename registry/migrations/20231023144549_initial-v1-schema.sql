CREATE SCHEMA v1;

CREATE TABLE v1.trunk_project
(
    name text PRIMARY KEY
);

CREATE TABLE v1.trunk_project_versions
(
    id SERIAL PRIMARY KEY,
    trunk_project_name text REFERENCES v1.trunk_project(name),
    version text NOT NULL,
    description text,
    repository_link text,
    documentation_link text,
    UNIQUE (trunk_project_name, version)
);

CREATE TABLE v1.extension_versions
(
    id SERIAL PRIMARY KEY,
    extension_name text NOT NULL,
    trunk_project_version_id int REFERENCES v1.trunk_project_versions(id),
    version text NOT NULL,
    UNIQUE (extension_name, trunk_project_version_id, version)
);

CREATE TABLE v1.extensions_loadable_libraries
(
    id SERIAL PRIMARY KEY,
    extension_version_id int REFERENCES v1.extension_versions(id),
    library_name text NOT NULL,
    requires_restart boolean DEFAULT false,
    priority int DEFAULT 2147483647 -- max value for the 4 byte Postgres integer
);

CREATE TABLE v1.extension_dependency
(
    id SERIAL PRIMARY KEY,
    extension_version_id int REFERENCES v1.extension_versions(id),
    depends_on_extension_name text NOT NULL
);

CREATE TABLE v1.extension_configurations
(
    id SERIAL PRIMARY KEY,
    extension_version_id int REFERENCES v1.extension_versions(id),
    is_required boolean DEFAULT false,
    configuration_name text NOT NULL,
    recommended_default_value text
);
