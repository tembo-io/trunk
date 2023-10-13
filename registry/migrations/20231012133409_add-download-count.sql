ALTER TABLE versions
ADD COLUMN download_count integer NOT NULL DEFAULT 0;

UPDATE versions
SET download_count = COALESCE(downloads, 0);

ALTER TABLE versions
DROP COLUMN downloads;

ALTER TABLE extensions
DROP COLUMN downloads;