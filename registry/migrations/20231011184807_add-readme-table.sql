CREATE TABLE IF NOT EXISTS readmes (
    id BIGSERIAL PRIMARY KEY,
    extension_id INT4 UNIQUE NOT NULL,
    readme_html TEXT NOT NULL
);
