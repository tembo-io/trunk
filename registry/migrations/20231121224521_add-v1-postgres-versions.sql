CREATE TABLE v1.postgres_version (
    major smallint UNIQUE,
    id SERIAL PRIMARY KEY
);

INSERT INTO v1.postgres_version(major) VALUES (14), (15), (16);

CREATE TABLE v1.trunk_project_postgres_support(
    id SERIAL PRIMARY KEY,
    postgres_version_id int REFERENCES v1.postgres_version(id),
    trunk_project_version_id int REFERENCES v1.trunk_project_versions(id)
);

INSERT INTO v1.trunk_project_postgres_support(postgres_version_id, trunk_project_version_id)
SELECT pg.id, tpv.id
FROM v1.trunk_project_versions tpv
JOIN v1.postgres_version pg ON pg.major = 15;