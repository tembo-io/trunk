-- Add migration script here
UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgroonga' AND extensions.id = versions.extension_id;
