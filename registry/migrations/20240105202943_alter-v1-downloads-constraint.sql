ALTER TABLE v1.trunk_project_downloads
DROP CONSTRAINT unique_downloads;

ALTER TABLE v1.trunk_project_downloads
ADD CONSTRAINT unique_downloads UNIQUE (platform_id, postgres_version_id, trunk_project_version_id);