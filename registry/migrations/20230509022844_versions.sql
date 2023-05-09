-- Change published_by column type to VARCHAR
ALTER TABLE versions
ALTER COLUMN published_by TYPE VARCHAR;