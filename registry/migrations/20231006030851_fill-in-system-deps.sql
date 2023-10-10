-- Note: auto-generated with https://gist.github.com/vrmiguel/fe63c1297d4f7ba5261af0610d17e36e

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'adminpack' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'aggs_for_vecs' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'amcheck' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'argm' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'auto_explain' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'autoinc' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'basebackup_to_shell' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'basic_archive' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'bloom' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libperl5.34","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'bool_plperl' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libperl5.34","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'bool_plperlu' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'btree_gist' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libcrypt1","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'chkpass' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'citext' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libpq5","openssl","libc6","liblz4-1","libzstd1","libssl3","libcurl4"]}'::jsonb
FROM extensions
WHERE extensions.name = 'citus' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libstdc++6","libc6","libgcc-s1"," libuuid1","libcurl4"]}'::jsonb
FROM extensions
WHERE extensions.name = 'clickhouse_fdw' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'count_distinct' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'credcheck' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'cube' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libpq5","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'dblink' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'dict_int' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'dict_xsyn' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"nix":["duckdb"]}'::jsonb
FROM extensions
WHERE extensions.name = 'duckdb_fdw' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'earthdistance' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'extra_window_functions' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'file_fdw' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'fuzzystrmatch' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'h3_pg' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'hstore' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libperl5.34"]}'::jsonb
FROM extensions
WHERE extensions.name = 'hstore_plperl' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libperl5.34"]}'::jsonb
FROM extensions
WHERE extensions.name = 'hstore_plperlu' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libpython3.10"]}'::jsonb
FROM extensions
WHERE extensions.name = 'hstore_plpython3u' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libzstd1","liblz4-1"]}'::jsonb
FROM extensions
WHERE extensions.name = 'hydra_columnar' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'hypopg' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'insert_username' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'intarray' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'ip4r' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'isn' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libpq5","libc6","openjdk-18-jre-headless"]}'::jsonb
FROM extensions
WHERE extensions.name = 'jdbc_fdw' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libperl5.34"]}'::jsonb
FROM extensions
WHERE extensions.name = 'jsonb_plperl' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libpython3.10"]}'::jsonb
FROM extensions
WHERE extensions.name = 'jsonb_plpython3u' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["librdkafka1","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'kafka_fdw' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'lo' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'lower_quantile' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'ltree' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'ltree_plpython3u' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libpython3.10"]}'::jsonb
FROM extensions
WHERE extensions.name = 'moddatetime' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'mysql_fdw' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'old_snapshot' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'oracle_fdw' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'orafce' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pageinspect' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'passwordcheck' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'periods' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_anonymize' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_bigm' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_buffercache' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libpq5","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_cron' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_currency' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_dirtyread' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libgcc-s1","libstdc++6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_embedding' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libpq5"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_failover_slots' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_financial' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_hashids' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_hint_plan' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_ivm' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_mon' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libcurl4","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_net' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_partman' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_prewarm' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_proctab' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_qualstats' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_rational' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_repack' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_roaringbitmap' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_semver' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_similarity' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_squeeze' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_stat_kcache' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_stat_monitor' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_stat_statements' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_surgery' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_timeit' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_tle' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_trgm' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_uuidv7' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_visibility' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_wait_sampling' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pg_walinspect' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgaudit' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["zlib1g","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgcrypto' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgfincore' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libpq5","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pglogical' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pglogical_ticker' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgq' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libgroonga0","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgroonga' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libstdc++6","libc6","libgcc-s1"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgrouting' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgrowlocks' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libsodium23","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgsodium' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libcurl4"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgsql_http' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libgdal30","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgsql_ogr_fdw' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgstattuple' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgtt' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["liburiparser1","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pguri' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pgvector' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libstdc++6","libgcc-s1","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'plcoffee' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pljava' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libperl5.34"]}'::jsonb
FROM extensions
WHERE extensions.name = 'plperl' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libperl5.34"]}'::jsonb
FROM extensions
WHERE extensions.name = 'plperlu' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'plpgsql' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'plpgsql_check' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'plprofiler' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libpq5","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'plproxy' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libpython3.10"]}'::jsonb
FROM extensions
WHERE extensions.name = 'plpython3u' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","r-base-core"]}'::jsonb
FROM extensions
WHERE extensions.name = 'plr' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libgcc-s1"]}'::jsonb
FROM extensions
WHERE extensions.name = 'plrust' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'plsh' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libtcl8.6.so","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pltcl' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libtcl8.6.so","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'pltclu' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libjson-c5","libgcc-s1","libpcre2-8-0","libstdc++6","libproj22","libxml2","libgdal30","libgeos-c1v5","libc6","libprotobuf-c1"]}'::jsonb
FROM extensions
WHERE extensions.name = 'postgis' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libprotobuf-c1"]}'::jsonb
FROM extensions
WHERE extensions.name = 'postgres_decoderbufs' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libpq5"]}'::jsonb
FROM extensions
WHERE extensions.name = 'postgres_fdw' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"pip":["xgboost"],"apt":["libpython3.10","libstdc++6","libgomp1","libopenblas0-pthread","libc6","libgcc-s1"]}'::jsonb
FROM extensions
WHERE extensions.name = 'postgresml' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'postgresql_anonymizer' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'postgresql_hll' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'postgresql_logfdw' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'postgresql_topn' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'postgresql_unit' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'prefix' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'quantile' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'random' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libfreetype6","libboost-serialization1.74.0","libc6","libstdc++6","libgcc-s1"]}'::jsonb
FROM extensions
WHERE extensions.name = 'rdkit' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libhiredis0.14"]}'::jsonb
FROM extensions
WHERE extensions.name = 'redis_fdw' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'refint' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'rum' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'seg' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libselinux1","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'sepgsql' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'sequential_uuids' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'set_user' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","openssl"]}'::jsonb
FROM extensions
WHERE extensions.name = 'sslinfo' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libssl3","openssl","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'sslutils' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'supautils' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'system_stats' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'tablefunc' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'tdigest' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libsybdb5","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'tds_fdw' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'temporal_tables' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'test_decoding' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libssl3","openssl","libpq5","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'timescaledb' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'tsm_system_time' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'unaccent' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libuuid1","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'uuid_ossp' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'wal2json' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libc6","libgcc-s1"]}'::jsonb
FROM extensions
WHERE extensions.name = 'wrappers' AND extensions.id = versions.extension_id;

UPDATE versions
SET system_dependencies = '{"apt":["libxml2","libc6"]}'::jsonb
FROM extensions
WHERE extensions.name = 'xml2' AND extensions.id = versions.extension_id;