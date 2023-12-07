INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'auth_delay', true
FROM v1.extension_versions
WHERE extension_name = 'auth_delay' AND version = '15.3.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'auto_explain', true
FROM v1.extension_versions
WHERE extension_name = 'auto_explain' AND version = '15.3.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'basebackup_to_shell', true
FROM v1.extension_versions
WHERE extension_name = 'basebackup_to_shell' AND version = '15.3.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'basic_archive', true
FROM v1.extension_versions
WHERE extension_name = 'basic_archive' AND version = '15.3.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart, priority)
SELECT id, 'citus', true, 0
FROM v1.extension_versions
WHERE extension_name = 'citus' AND version = '12.0.1';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'credcheck', true
FROM v1.extension_versions
WHERE extension_name = 'credcheck' AND version = '2.0.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'passwordcheck', true
FROM v1.extension_versions
WHERE extension_name = 'passwordcheck' AND version = '15.3.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_anonymize', true
FROM v1.extension_versions
WHERE extension_name = 'pg_anonymize' AND version = '1.0.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pgaudit', true
FROM v1.extension_versions
WHERE extension_name = 'pgaudit' AND version = '1.7.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_cron', true
FROM v1.extension_versions
WHERE extension_name = 'pg_cron' AND version = '1.5.2';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_failover_slots', true
FROM v1.extension_versions
WHERE extension_name = 'pg_failover_slots' AND version = '1.0.1';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_later', true
FROM v1.extension_versions
WHERE extension_name = 'pg_later' AND version = '0.0.12';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pglogical', true
FROM v1.extension_versions
WHERE extension_name = 'pglogical' AND version = '2.4.3';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pglogical_ticker', true
FROM v1.extension_versions
WHERE extension_name = 'pglogical_ticker' AND version = '1.4.1';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_net', true
FROM v1.extension_versions
WHERE extension_name = 'pg_net' AND version = '0.7.1';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_partman_bgw', true
FROM v1.extension_versions
WHERE extension_name = 'pg_partman' AND version = '4.7.4';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_prewarm', true
FROM v1.extension_versions
WHERE extension_name = 'pg_prewarm' AND version = '1.2.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_qualstats', true
FROM v1.extension_versions
WHERE extension_name = 'pg_qualstats' AND version = '2.0.4';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_stat_kcache', true
FROM v1.extension_versions
WHERE extension_name = 'pg_stat_kcache' AND version = '2.2.1';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_stat_monitor', true
FROM v1.extension_versions
WHERE extension_name = 'pg_stat_monitor' AND version = '2.0.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_tle', true
FROM v1.extension_versions
WHERE extension_name = 'pg_tle' AND version = '1.0.4';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_uuidv7', true
FROM v1.extension_versions
WHERE extension_name = 'pg_uuidv7' AND version = '1.0.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pg_wait_sampling', true
FROM v1.extension_versions
WHERE extension_name = 'pg_wait_sampling' AND version = '1.1.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'pgml', true
FROM v1.extension_versions
WHERE extension_name = 'pgml' AND version = '2.7.1';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'plrust', true
FROM v1.extension_versions
WHERE extension_name = 'plrust' AND version = '1.2.3';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'postgresql_anonymizer', true
FROM v1.extension_versions
WHERE extension_name = 'postgresql_anonymizer' AND version = '1.1.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'sepgsql', true
FROM v1.extension_versions
WHERE extension_name = 'sepgsql' AND version = '15.3.0';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'set_user', true
FROM v1.extension_versions
WHERE extension_name = 'set_user' AND version = '4.0.1';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'supautils', true
FROM v1.extension_versions
WHERE extension_name = 'supautils' AND version = '1.7.3';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart, priority)
SELECT id, 'timescaledb', true, 0
FROM v1.extension_versions
WHERE extension_name = 'timescaledb' AND version = '2.11.1';

INSERT INTO v1.extensions_loadable_libraries(extension_version_id, library_name, requires_restart)
SELECT id, 'vectorize', true
FROM v1.extension_versions
WHERE extension_name = 'vectorize' AND version = '0.2.0';
