-- Add table extension_owners
CREATE TABLE IF NOT EXISTS extension_owners (
    extension_id INT4 NOT NULL,
    owner_id VARCHAR NOT NULL,
    created_at TIMESTAMP,
    created_by VARCHAR,
    deleted BOOL NOT NULL DEFAULT false
);
