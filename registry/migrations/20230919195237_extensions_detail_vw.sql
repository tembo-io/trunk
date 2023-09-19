-- query that returns the latest version of each extension
create or replace view extension_detail_vw as 
WITH latest_versions AS (
    SELECT
        v.extension_id,
        v.num AS latest_version,
        v.license,
        ROW_NUMBER() OVER(PARTITION BY v.extension_id ORDER BY v.updated_at DESC) AS rn
    FROM public.versions v
)
SELECT
    e."name" AS extension_name,
    lv.latest_version,
    e.created_at,
    e.updated_at,
    e.description,
    e.homepage,
    e.documentation,
    e.repository,
    lv.license,
    jsonb_agg(DISTINCT eo.user_name) AS owners,
    jsonb_agg(DISTINCT cg.name) AS categories
FROM public.extensions e
LEFT JOIN latest_versions lv ON e.id = lv.extension_id AND lv.rn = 1
LEFT JOIN public.extension_owners eo ON e.id = eo.extension_id AND eo.deleted = false
LEFT JOIN public.extensions_categories ec ON e.id = ec.extension_id
left join public.categories cg on ec.category_id  = cg.id
GROUP BY 
    e."name", 
    lv.latest_version, 
    e.created_at, 
    e.updated_at, 
    e.description, 
    e.homepage, 
    e.documentation, 
    e.repository, 
    lv.license
