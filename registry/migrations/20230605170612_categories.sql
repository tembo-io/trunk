-- Add categories table
CREATE TABLE IF NOT EXISTS categories (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR,
    slug VARCHAR,
    description VARCHAR,
    extension_count INT4,
    created_at TIMESTAMP
);
