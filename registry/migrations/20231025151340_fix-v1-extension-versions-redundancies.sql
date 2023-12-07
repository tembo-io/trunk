-- Remove all previous entries since they contained many duplicates
DELETE from v1.extension_versions;

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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_later',
  tpv.id as trunk_project_version_id,
  '0.0.12'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_later'
LIMIT 1
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
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgmq',
  tpv.id as trunk_project_version_id,
  '0.32.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pgmq'
LIMIT 1
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
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'vectorize',
  tpv.id as trunk_project_version_id,
  '0.2.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'vectorize'
LIMIT 1
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
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pglogical',
  tpv.id as trunk_project_version_id,
  '2.4.3'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pglogical'
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pglogical_origin',
  tpv.id as trunk_project_version_id,
  '1.0.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pglogical'
LIMIT 1
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
LIMIT 1
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
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pgml',
  tpv.id as trunk_project_version_id,
  '2.7.1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgresml'
LIMIT 1
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
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'citus',
  tpv.id as trunk_project_version_id,
  '12.0-1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'citus'
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'citus_columnar',
  tpv.id as trunk_project_version_id,
  '11.3-1'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'citus'
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
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
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'address_standardizer',
  tpv.id as trunk_project_version_id,
  '3.4.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgis'
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'address_standardizer_data_us',
  tpv.id as trunk_project_version_id,
  '3.4.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgis'
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'postgis',
  tpv.id as trunk_project_version_id,
  '3.4.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgis'
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'postgis_raster',
  tpv.id as trunk_project_version_id,
  '3.4.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgis'
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'postgis_tiger_geocoder',
  tpv.id as trunk_project_version_id,
  '3.4.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgis'
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'postgis_topology',
  tpv.id as trunk_project_version_id,
  '3.4.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'postgis'
LIMIT 1
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
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'plcoffee',
  tpv.id as trunk_project_version_id,
  '3.1.7'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'plcoffee'
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                

INSERT INTO v1.extension_versions (extension_name, trunk_project_version_id, version)
SELECT 
  'pg_bm25',
  tpv.id as trunk_project_version_id,
  '0.0.0'
FROM 
  v1.trunk_project_versions tpv
WHERE 
  tpv.trunk_project_name = 'pg_bm25'
LIMIT 1
ON CONFLICT (extension_name, trunk_project_version_id, version) 
DO NOTHING;
                
