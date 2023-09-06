-- Add libraries column to versions table
ALTER TABLE versions
ADD COLUMN libraries jsonb;