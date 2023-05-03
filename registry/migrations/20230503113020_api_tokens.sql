-- Add table api_tokens
CREATE TABLE IF NOT EXISTS api_tokens (
    id BIGSERIAL PRIMARY KEY,
    user_id VARCHAR,
    token BYTEA,
    name VARCHAR,
    created_at TIMESTAMP,
    last_used_at TIMESTAMP
);