-- Add postgres_version column to versions table
ALTER TABLE versions
ADD COLUMN postgres_version INT4 NOT NULL DEFAULT 15;
