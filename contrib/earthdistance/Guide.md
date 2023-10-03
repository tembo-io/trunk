# What is this extension?
> *Calculate Earth distances.*

`earthdistance` is a C-based extension that provides the capability to calculate great-circle distances on the Earth's surface between two given points. It's important to note that it assumes that Earth is a perfect sphere.

# When should you use it?

This extension can be used to calculate the approximate distance between two points on the Earth's surface using their latitude and longitude coordinates. If high accuracy is important, consider using the PostGIS extension.

# Example use case.

**Archeological Expeditions:** 

In a fictitious scenario, an archeologist is studying newly-discovered ancient site within the Amazon rainforest. Having already been researching another ruin nearby, she's keen to measure the distance between the two locations. As she and her team are remote without access to the internet, she uses her local PostgreSQL installation on her laptop to compute the distance.

# Example test script.

```
-- Create example table, excavation_sites, and populate with test data
CREATE TABLE excavation_sites (
    site_id serial PRIMARY KEY,
    site_name text,
    location point  -- (latitude, longitude)
);

INSERT INTO excavation_sites (site_name, location) VALUES
('Previous Ruin Site', '(-3.247598, -62.692215)'),
('New Discovery Site', '(-2.950692, -63.224320)');

-- Calculate the distance between sites
SELECT
    a.site_name AS site_a,
    b.site_name AS site_b,
    earth_distance(ll_to_earth(a.location[0], a.location[1]), ll_to_earth(b.location[0], b.location[1])) AS distance_meters
FROM
    excavation_sites a,
    excavation_sites b
WHERE
    a.site_name = 'Previous Ruin Site' AND b.site_name = 'New Discovery Site';

       site_a       |       site_b       |  distance_meters
--------------------+--------------------+-------------------
 Previous Ruin Site | New Discovery Site | 67755.47316151766
(1 row)

-- Drop table(s) and affiliate items to complete example
DROP TABLE excavation_sites CASCADE ;
```
