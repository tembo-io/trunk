-- Add user_name column to api_tokens
ALTER TABLE api_tokens
ADD COLUMN user_name VARCHAR;
UPDATE api_tokens SET user_name = '' WHERE user_name IS NULL;

ALTER TABLE api_tokens
ALTER COLUMN user_name SET NOT NULL;

-- Add user_name column to extension_owners
ALTER TABLE extension_owners
ADD COLUMN user_name VARCHAR;
UPDATE extension_owners SET user_name = '' WHERE user_name IS NULL;

ALTER TABLE extension_owners
ALTER COLUMN user_name SET NOT NULL;