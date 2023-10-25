-- jsonb_plpython3u v1.0 depends on [plpython3u]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plpython3u' FROM v1.extension_versions
WHERE extension_name = 'jsonb_plpython3u' AND version = '1.0';

-- ltree_plpython3u v1.0 depends on [ltree, plpython3u]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'ltree' FROM v1.extension_versions
WHERE extension_name = 'ltree_plpython3u' AND version = '1.0';
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plpython3u' FROM v1.extension_versions
WHERE extension_name = 'ltree_plpython3u' AND version = '1.0';

-- bool_plperl v1.0 depends on [plperl]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plperl' FROM v1.extension_versions
WHERE extension_name = 'bool_plperl' AND version = '1.0';

-- pgcozy v1.0 depends on [pg_prewarm, pg_buffercache]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'pg_prewarm' FROM v1.extension_versions
WHERE extension_name = 'pgcozy' AND version = '1.0';
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'pg_buffercache' FROM v1.extension_versions
WHERE extension_name = 'pgcozy' AND version = '1.0';

-- jsonb_plperl v1.0 depends on [plperl]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plperl' FROM v1.extension_versions
WHERE extension_name = 'jsonb_plperl' AND version = '1.0';

-- bool_plperlu v1.0 depends on [plperlu]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plperlu' FROM v1.extension_versions
WHERE extension_name = 'bool_plperlu' AND version = '1.0';

-- hstore_plperlu v1.0 depends on [hstore, plperlu]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'hstore' FROM v1.extension_versions
WHERE extension_name = 'hstore_plperlu' AND version = '1.0';
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plperlu' FROM v1.extension_versions
WHERE extension_name = 'hstore_plperlu' AND version = '1.0';

-- pgjwt v0.2.0 depends on [pgcrypto]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'pgcrypto' FROM v1.extension_versions
WHERE extension_name = 'pgjwt' AND version = '0.2.0';

-- ora_migrator v1.0.0 depends on [oracle_fdw, db_migrator]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'oracle_fdw' FROM v1.extension_versions
WHERE extension_name = 'ora_migrator' AND version = '1.0.0';
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'db_migrator' FROM v1.extension_versions
WHERE extension_name = 'ora_migrator' AND version = '1.0.0';

-- hstore_plperl v1.0 depends on [hstore, plperl]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'hstore' FROM v1.extension_versions
WHERE extension_name = 'hstore_plperl' AND version = '1.0';
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plperl' FROM v1.extension_versions
WHERE extension_name = 'hstore_plperl' AND version = '1.0';

-- hstore_plpython3u v1.0 depends on [hstore, plpython3u]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'hstore' FROM v1.extension_versions
WHERE extension_name = 'hstore_plpython3u' AND version = '1.0';
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plpython3u' FROM v1.extension_versions
WHERE extension_name = 'hstore_plpython3u' AND version = '1.0';

-- pglogical_ticker v1.4 depends on [pglogical]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'pglogical' FROM v1.extension_versions
WHERE extension_name = 'pglogical_ticker' AND version = '1.4';

-- currency v0.0.3 depends on [plpgsql]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plpgsql' FROM v1.extension_versions
WHERE extension_name = 'currency' AND version = '0.0.3';

-- anon v1.1.0 depends on [pgcrypto]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'pgcrypto' FROM v1.extension_versions
WHERE extension_name = 'anon' AND version = '1.1.0';

-- jsonb_plperlu v1.0 depends on [plperlu]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plperlu' FROM v1.extension_versions
WHERE extension_name = 'jsonb_plperlu' AND version = '1.0';

-- pgbouncer_fdw v1.0.1 depends on [dblink]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'dblink' FROM v1.extension_versions
WHERE extension_name = 'pgbouncer_fdw' AND version = '1.0.1';

-- earthdistance v1.1 depends on [cube]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'cube' FROM v1.extension_versions
WHERE extension_name = 'earthdistance' AND version = '1.1';

-- pg_stat_kcache v2.2.1 depends on [pg_stat_statements]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'pg_stat_statements' FROM v1.extension_versions
WHERE extension_name = 'pg_stat_kcache' AND version = '2.2.1';

-- h3_postgis v4.1.3 depends on [h3, postgis, postgis_raster]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'h3' FROM v1.extension_versions
WHERE extension_name = 'h3_postgis' AND version = '4.1.3';
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'postgis' FROM v1.extension_versions
WHERE extension_name = 'h3_postgis' AND version = '4.1.3';
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'postgis_raster' FROM v1.extension_versions
WHERE extension_name = 'h3_postgis' AND version = '4.1.3';

-- pgtap v1.2.0 depends on [plpgsql]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plpgsql' FROM v1.extension_versions
WHERE extension_name = 'pgtap' AND version = '1.2.0';

-- periods v1.2 depends on [btree_gist]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'btree_gist' FROM v1.extension_versions
WHERE extension_name = 'periods' AND version = '1.2';

-- pg_later v0.0.12 depends on [pgmq]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'pgmq' FROM v1.extension_versions
WHERE extension_name = 'pg_later' AND version = '0.0.12';

-- plpgsql_check v2.3 depends on [plpgsql]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plpgsql' FROM v1.extension_versions
WHERE extension_name = 'plpgsql_check' AND version = '2.3';

-- pgrouting v3.5.1 depends on [plpgsql, postgis]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'plpgsql' FROM v1.extension_versions
WHERE extension_name = 'pgrouting' AND version = '3.5.1';
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'postgis' FROM v1.extension_versions
WHERE extension_name = 'pgrouting' AND version = '3.5.1';

-- vectorize v0.2.0 depends on [pg_cron, pgmq, vector]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'pg_cron' FROM v1.extension_versions
WHERE extension_name = 'vectorize' AND version = '0.2.0';
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'pgmq' FROM v1.extension_versions
WHERE extension_name = 'vectorize' AND version = '0.2.0';
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'vector' FROM v1.extension_versions
WHERE extension_name = 'vectorize' AND version = '0.2.0';

-- postgis_raster v3.4.0 depends on [postgis]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'postgis' FROM v1.extension_versions
WHERE extension_name = 'postgis_raster' AND version = '3.4.0';

-- postgis_tiger_geocoder v3.4.0 depends on [postgis, fuzzystrmatch]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'postgis' FROM v1.extension_versions
WHERE extension_name = 'postgis_tiger_geocoder' AND version = '3.4.0';
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'fuzzystrmatch' FROM v1.extension_versions
WHERE extension_name = 'postgis_tiger_geocoder' AND version = '3.4.0';

-- postgis_topology v3.4.0 depends on [postgis]
INSERT INTO v1.extension_dependency (extension_version_id, depends_on_extension_name)
SELECT id, 'postgis' FROM v1.extension_versions
WHERE extension_name = 'postgis_topology' AND version = '3.4.0';
