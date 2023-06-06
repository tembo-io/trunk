-- Add categories table
CREATE TABLE IF NOT EXISTS categories (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR,
    slug VARCHAR,
    description VARCHAR,
    extension_count INT4,
    created_at TIMESTAMP
);

-- Add extensions_categories table
CREATE TABLE IF NOT EXISTS extensions_categories (
    extension_id INT4,
    category_id INT4
);
