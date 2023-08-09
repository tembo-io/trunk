-- Add extension_name column to versions
ALTER TABLE versions
ADD COLUMN extension_name VARCHAR;
UPDATE versions SET extension_name = '' WHERE extension_name IS NULL;

ALTER TABLE versions
ALTER COLUMN extension_name SET NOT NULL;
