-- Add migration script here

ALTER TABLE shared_preload_libraries
ADD COLUMN name VARCHAR;
