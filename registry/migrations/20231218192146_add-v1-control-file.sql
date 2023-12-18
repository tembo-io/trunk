-- Create control_file table
CREATE TABLE IF NOT EXISTS v1.control_file
(
    id SERIAL PRIMARY KEY,
    extension_version_id int REFERENCES v1.extension_versions(id),
    absent boolean DEFAULT false,
    content text
);
