-- Fill in v1.trunk_project using current values for `extensions`
INSERT INTO v1.trunk_project(name)
SELECT DISTINCT name FROM extensions;

-- Fill in v1.trunk_project_versions using current values from `extensions` and `versions`
INSERT INTO v1.trunk_project_versions(trunk_project_name, version, description, repository_link, documentation_link)
SELECT e.name, v.num, e.description, e.repository, e.documentation
FROM extensions e
JOIN versions v ON e.id = v.extension_id;