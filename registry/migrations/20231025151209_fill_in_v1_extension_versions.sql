-- The 10 first queries were written manually since those extensions do not have a control file
INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'auto_explain',
  tpv.id as trunk_project_version_id,
  '15.3.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'auto_explain'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'basebackup_to_shell',
  tpv.id as trunk_project_version_id,
  '15.3.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'basebackup_to_shell'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'basic_archive',
  tpv.id as trunk_project_version_id,
  '15.3.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'basic_archive'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;


INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_anonymize',
  tpv.id as trunk_project_version_id,
  '1.0.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_anonymize'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'passwordcheck',
  tpv.id as trunk_project_version_id,
  '15.3.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'passwordcheck'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_failover_slots',
  tpv.id as trunk_project_version_id,
  '1.0.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_failover_slots'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'sepgsql',
  tpv.id as trunk_project_version_id,
  '15.3.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'sepgsql'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'wal2json',
  tpv.id as trunk_project_version_id,
  '2.5.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'wal2json'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'auth_delay',
  tpv.id as trunk_project_version_id,
  '15.3.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'auth_delay'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'test_decoding',
  tpv.id as trunk_project_version_id,
  '15.3.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'test_decoding'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;


-- The rest of the queries from this point onwards were generated automatically by https://gist.github.com/vrmiguel/ef2925a31fa8685435f3c3dd8fe481b1
INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'adminpack',
  tpv.id as trunk_project_version_id,
  '2.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'adminpack'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'hstore_plpython3u',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'hstore_plpython3u'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgcozy',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgcozy'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_track_settings',
  tpv.id as trunk_project_version_id,
  '2.1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_track_settings'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'citext',
  tpv.id as trunk_project_version_id,
  '1.6'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'citext'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'amcheck',
  tpv.id as trunk_project_version_id,
  '1.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'amcheck'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'unaccent',
  tpv.id as trunk_project_version_id,
  '1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'unaccent'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'ora_migrator',
  tpv.id as trunk_project_version_id,
  '1.0.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'ora_migrator'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'refint',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'refint'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'btree_gin',
  tpv.id as trunk_project_version_id,
  '1.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'btree_gin'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_uuidv7',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_uuidv7'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'seg',
  tpv.id as trunk_project_version_id,
  '1.4'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'seg'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'count_distinct',
  tpv.id as trunk_project_version_id,
  '3.0.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'count_distinct'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'db_migrator',
  tpv.id as trunk_project_version_id,
  '1.0.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'db_migrator'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'plperlu',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'plperlu'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'quantile',
  tpv.id as trunk_project_version_id,
  '1.1.7'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'quantile'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_dirtyread',
  tpv.id as trunk_project_version_id,
  '2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_dirtyread'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'sslinfo',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'sslinfo'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'intarray',
  tpv.id as trunk_project_version_id,
  '1.5'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'intarray'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgstattuple',
  tpv.id as trunk_project_version_id,
  '1.5'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgstattuple'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'random',
  tpv.id as trunk_project_version_id,
  '1.0.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'random'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'financial',
  tpv.id as trunk_project_version_id,
  '1.0.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_financial'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'aggs_for_vecs',
  tpv.id as trunk_project_version_id,
  '1.3.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'aggs_for_vecs'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;


INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgrowlocks',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgrowlocks'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;


INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'bloom',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'bloom'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'plpgsql',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'plpgsql'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgjwt',
  tpv.id as trunk_project_version_id,
  '0.2.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgjwt'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'dict_xsyn',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'dict_xsyn'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'jsonb_plperlu',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'jsonb_plperlu'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'sequential_uuids',
  tpv.id as trunk_project_version_id,
  '1.0.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'sequential_uuids'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'meta',
  tpv.id as trunk_project_version_id,
  '0.3.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'meta'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'earthdistance',
  tpv.id as trunk_project_version_id,
  '1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'earthdistance'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_stat_statements',
  tpv.id as trunk_project_version_id,
  '1.10'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_stat_statements'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pltclu',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pltclu'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'sslutils',
  tpv.id as trunk_project_version_id,
  '1.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'sslutils'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pageinspect',
  tpv.id as trunk_project_version_id,
  '1.11'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pageinspect'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'moddatetime',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'moddatetime'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'tcn',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'tcn'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'jdbc_fdw',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'jdbc_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'plpython3u',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'plpython3u'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'hstore_plperlu',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'hstore_plperlu'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'tsm_system_rows',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'tsm_system_rows'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_prewarm',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_prewarm'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'dict_int',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'dict_int'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;


INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'intagg',
  tpv.id as trunk_project_version_id,
  '1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'intagg'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_visibility',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_visibility'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'old_snapshot',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'old_snapshot'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'tsm_system_time',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'tsm_system_time'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_text_semver',
  tpv.id as trunk_project_version_id,
  '0.1.4'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_text_semver'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'topn',
  tpv.id as trunk_project_version_id,
  '2.4.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgresql_topn'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;


INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgcrypto',
  tpv.id as trunk_project_version_id,
  '1.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgcrypto'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'file_fdw',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'file_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'xml2',
  tpv.id as trunk_project_version_id,
  '1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'xml2'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'ltree',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'ltree'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'hstore_plperl',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'hstore_plperl'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'hstore',
  tpv.id as trunk_project_version_id,
  '1.8'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'hstore'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'tablefunc',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'tablefunc'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'insert_username',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'insert_username'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_surgery',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_surgery'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'credcheck',
  tpv.id as trunk_project_version_id,
  '2.0.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'credcheck'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;


INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_buffercache',
  tpv.id as trunk_project_version_id,
  '1.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_buffercache'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'lo',
  tpv.id as trunk_project_version_id,
  '1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'lo'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'uuid-ossp',
  tpv.id as trunk_project_version_id,
  '1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'uuid_ossp'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'ltree_plpython3u',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'ltree_plpython3u'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_dbms_job',
  tpv.id as trunk_project_version_id,
  '1.5.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_dbms_job'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'supautils',
  tpv.id as trunk_project_version_id,
  '0.1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'supautils'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_walinspect',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_walinspect'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgbouncer_fdw',
  tpv.id as trunk_project_version_id,
  '1.0.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgbouncer_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_permissions',
  tpv.id as trunk_project_version_id,
  '1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_permissions'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'anon',
  tpv.id as trunk_project_version_id,
  '1.1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgresql_anonymizer'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'jsonb_plpython3u',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'jsonb_plpython3u'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'chkpass',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'chkpass'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'jsonb_plperl',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'jsonb_plperl'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'autoinc',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'autoinc'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'meta_triggers',
  tpv.id as trunk_project_version_id,
  '0.3.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'meta_triggers'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_trgm',
  tpv.id as trunk_project_version_id,
  '1.6'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_trgm'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'extra_window_functions',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'extra_window_functions'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;


INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pglogical_ticker',
  tpv.id as trunk_project_version_id,
  '1.4'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pglogical_ticker'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'cube',
  tpv.id as trunk_project_version_id,
  '1.5'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'cube'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;


INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_repack',
  tpv.id as trunk_project_version_id,
  '1.4.8'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_repack'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'uri',
  tpv.id as trunk_project_version_id,
  '1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pguri'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'currency',
  tpv.id as trunk_project_version_id,
  '0.0.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_currency'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'prefix',
  tpv.id as trunk_project_version_id,
  '1.2.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'prefix'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'isn',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'isn'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'ogr_fdw',
  tpv.id as trunk_project_version_id,
  '1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgsql_ogr_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'emailaddr',
  tpv.id as trunk_project_version_id,
  '0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgemailaddr'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'timeit',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_timeit'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'log_fdw',
  tpv.id as trunk_project_version_id,
  '1.4'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgresql_logfdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;


INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgfincore',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgfincore'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'dblink',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'dblink'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pltcl',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pltcl'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgtap',
  tpv.id as trunk_project_version_id,
  '1.2.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgtap'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_cron',
  tpv.id as trunk_project_version_id,
  '1.5'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_cron'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_wait_sampling',
  tpv.id as trunk_project_version_id,
  '1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_wait_sampling'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'postgres_fdw',
  tpv.id as trunk_project_version_id,
  '1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgres_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'temporal_tables',
  tpv.id as trunk_project_version_id,
  '1.2.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'temporal_tables'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;


INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'decoderbufs',
  tpv.id as trunk_project_version_id,
  '0.1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgres_decoderbufs'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'mysql_fdw',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'mysql_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_stat_kcache',
  tpv.id as trunk_project_version_id,
  '2.2.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_stat_kcache'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'hll',
  tpv.id as trunk_project_version_id,
  '2.16'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgresql_hll'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'http',
  tpv.id as trunk_project_version_id,
  '1.5'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgsql_http'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'rum',
  tpv.id as trunk_project_version_id,
  '1.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'rum'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'plr',
  tpv.id as trunk_project_version_id,
  '8.4.5'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'plr'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_freespacemap',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_freespacemap'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'plsh',
  tpv.id as trunk_project_version_id,
  '2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'plsh'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'columnar',
  tpv.id as trunk_project_version_id,
  '11.1-7'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'hydra_columnar'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_squeeze',
  tpv.id as trunk_project_version_id,
  '1.5'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_squeeze'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'embedding',
  tpv.id as trunk_project_version_id,
  '0.2.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_embedding'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_mon',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_mon'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_qualstats',
  tpv.id as trunk_project_version_id,
  '2.0.4'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_qualstats'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'system_stats',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'system_stats'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_tle',
  tpv.id as trunk_project_version_id,
  '1.0.4'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_tle'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_ivm',
  tpv.id as trunk_project_version_id,
  '1.5'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_ivm'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgaudit',
  tpv.id as trunk_project_version_id,
  '1.7'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgaudit'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'duckdb_fdw',
  tpv.id as trunk_project_version_id,
  '1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'duckdb_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'bool_plperlu',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'bool_plperlu'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgq',
  tpv.id as trunk_project_version_id,
  '3.5'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgq'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'bool_plperl',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'bool_plperl'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_stat_monitor',
  tpv.id as trunk_project_version_id,
  '2.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_stat_monitor'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'hypopg',
  tpv.id as trunk_project_version_id,
  '1.4.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'hypopg'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'h3',
  tpv.id as trunk_project_version_id,
  '4.1.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'h3_pg'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'h3_postgis',
  tpv.id as trunk_project_version_id,
  '4.1.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'h3_pg'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'periods',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'periods'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'plproxy',
  tpv.id as trunk_project_version_id,
  '2.10.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'plproxy'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'plperl',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'plperl'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'fuzzystrmatch',
  tpv.id as trunk_project_version_id,
  '1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'fuzzystrmatch'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'lower_quantile',
  tpv.id as trunk_project_version_id,
  '1.0.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'lower_quantile'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'zhparser',
  tpv.id as trunk_project_version_id,
  '2.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'zhparser'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'ip4r',
  tpv.id as trunk_project_version_id,
  '2.4'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'ip4r'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_net',
  tpv.id as trunk_project_version_id,
  '0.7.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_net'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'tds_fdw',
  tpv.id as trunk_project_version_id,
  '2.0.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'tds_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;


INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_jsonschema',
  tpv.id as trunk_project_version_id,
  '0.1.4'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_jsonschema'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'redis_fdw',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'redis_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_rational',
  tpv.id as trunk_project_version_id,
  '0.0.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_rational'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'roaringbitmap',
  tpv.id as trunk_project_version_id,
  '0.5'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_roaringbitmap'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'plprofiler',
  tpv.id as trunk_project_version_id,
  '4.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'plprofiler'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_bigm',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_bigm'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'clerk_fdw',
  tpv.id as trunk_project_version_id,
  '0.2.4'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'clerk_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'btree_gist',
  tpv.id as trunk_project_version_id,
  '1.7'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'btree_gist'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'tdigest',
  tpv.id as trunk_project_version_id,
  '1.4.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'tdigest'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'semver',
  tpv.id as trunk_project_version_id,
  '0.32.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_semver'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'oracle_fdw',
  tpv.id as trunk_project_version_id,
  '1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'oracle_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'argm',
  tpv.id as trunk_project_version_id,
  '1.1.2'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'argm'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_hashids',
  tpv.id as trunk_project_version_id,
  '1.2.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_hashids'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_proctab',
  tpv.id as trunk_project_version_id,
  '0.0.10'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_proctab'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgsodium',
  tpv.id as trunk_project_version_id,
  '3.1.8'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgsodium'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_hint_plan',
  tpv.id as trunk_project_version_id,
  '1.5'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_hint_plan'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'wrappers',
  tpv.id as trunk_project_version_id,
  '0.1.15'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'wrappers'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'vector',
  tpv.id as trunk_project_version_id,
  '0.5.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgvector'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_graphql',
  tpv.id as trunk_project_version_id,
  '1.4.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_graphql'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgtt',
  tpv.id as trunk_project_version_id,
  '2.10.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgtt'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'plrust',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'plrust'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'set_user',
  tpv.id as trunk_project_version_id,
  '4.0.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'set_user'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_similarity',
  tpv.id as trunk_project_version_id,
  '1.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_similarity'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_later',
  tpv.id as trunk_project_version_id,
  '0.0.11'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_later'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_partman',
  tpv.id as trunk_project_version_id,
  '4.7.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_partman'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'timescaledb',
  tpv.id as trunk_project_version_id,
  '2.11.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'timescaledb'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgl_ddl_deploy',
  tpv.id as trunk_project_version_id,
  '2.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgl_ddl_deploy'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'orafce',
  tpv.id as trunk_project_version_id,
  '4.4'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'orafce'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'clickhouse_fdw',
  tpv.id as trunk_project_version_id,
  '1.4'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'clickhouse_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pljava',
  tpv.id as trunk_project_version_id,
  '1.6.5'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pljava'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgrouting',
  tpv.id as trunk_project_version_id,
  '3.5.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgrouting'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'kafka_fdw',
  tpv.id as trunk_project_version_id,
  '0.0.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'kafka_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgmq',
  tpv.id as trunk_project_version_id,
  '0.32.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgmq'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'plpgsql_check',
  tpv.id as trunk_project_version_id,
  '2.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'plpgsql_check'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING; 

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'prometheus_fdw',
  tpv.id as trunk_project_version_id,
  '0.1.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'prometheus_fdw'
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;