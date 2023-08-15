-- Add extension_name column to versions table
ALTER TABLE versions
ADD COLUMN extension_name VARCHAR;
UPDATE versions SET extension_name = e.name FROM extensions e WHERE versions.extension_id = e.id AND versions.extension_name IS NULL;
