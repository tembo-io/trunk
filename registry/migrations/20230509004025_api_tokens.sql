-- Add user_name column to api_tokens
ALTER TABLE api_tokens
ADD COLUMN user_name VARCHAR;

-- Add user_name column to extension_owners
ALTER TABLE extension_owners
ADD COLUMN user_name VARCHAR;
