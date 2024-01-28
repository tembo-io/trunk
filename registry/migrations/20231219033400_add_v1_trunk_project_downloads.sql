CREATE TABLE v1.trunk_project_downloads(
    id SERIAL PRIMARY KEY,
    platform_id int REFERENCES v1.platform(id),
    postgres_version_id int REFERENCES v1.postgres_version(id),
    trunk_project_version_id int REFERENCES v1.trunk_project_versions(id),
    download_url TEXT NOT NULL,
    download_count int NOT NULL DEFAULT 0,
    sha256 TEXT NOT NULL
);