-- Add migration script here

ALTER TABLE versions
ADD COLUMN system_dependencies jsonb;