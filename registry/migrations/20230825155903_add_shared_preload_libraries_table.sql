-- Add migration script here
CREATE TABLE IF NOT EXISTS shared_preload_libraries (
    name VARCHAR NOT NULL
);

-- Add extension names
INSERT INTO shared_preload_libraries(name)
VALUES ('auth_delay');
INSERT INTO shared_preload_libraries(name)
VALUES ('auto_explain');
INSERT INTO shared_preload_libraries(name)
VALUES ('basebackup_to_shell');
INSERT INTO shared_preload_libraries(name)
VALUES ('basic_archive');
INSERT INTO shared_preload_libraries(name)
VALUES ('citus');
INSERT INTO shared_preload_libraries(name)
VALUES ('credcheck');
INSERT INTO shared_preload_libraries(name)
VALUES ('passwordcheck');
INSERT INTO shared_preload_libraries(name)
VALUES ('pg_anonymize');
INSERT INTO shared_preload_libraries(name)
VALUES ('pgaudit');
INSERT INTO shared_preload_libraries(name)
VALUES ('pg_cron');
INSERT INTO shared_preload_libraries(name)
VALUES ('pg_failover_slots');
INSERT INTO shared_preload_libraries(name)
VALUES ('pg_later');
INSERT INTO shared_preload_libraries(name)
VALUES ('pglogical');
INSERT INTO shared_preload_libraries(name)
VALUES ('pg_net');
INSERT INTO shared_preload_libraries(name)
VALUES ('pg_stat_kcache');
INSERT INTO shared_preload_libraries(name)
VALUES ('pg_tle');
INSERT INTO shared_preload_libraries(name)
VALUES ('plrust');
INSERT INTO shared_preload_libraries(name)
VALUES ('postgresql_anonymizer');
INSERT INTO shared_preload_libraries(name)
VALUES ('sepgsql');
INSERT INTO shared_preload_libraries(name)
VALUES ('supautils');
INSERT INTO shared_preload_libraries(name)
VALUES ('timescaledb');
INSERT INTO shared_preload_libraries(name)
VALUES ('vectorize');
