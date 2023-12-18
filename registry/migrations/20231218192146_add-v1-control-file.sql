-- Create control_file table
CREATE TABLE IF NOT EXISTS v1.control_file
(
    id SERIAL PRIMARY KEY,
    extension_version_id int REFERENCES v1.extension_versions(id),
    absent boolean DEFAULT false,
    content text DEFAULT ''
);

-- For each record in v1.extension_versions, create a record in v1.control_file
INSERT INTO v1.control_file (extension_version_id)
SELECT id FROM v1.extension_versions;
