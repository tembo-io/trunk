# **What is this extension?**

> Bloom-filter-based index access.

`bloom` is a C-based extension that allows for index access based on Bloom filters. Bloom filters are space-efficient probabilistic data structures used to test whether an element is a member of a set. It allows for false positive matches but eliminates false negatives.

# **When should you use it?**

This extension may be used when considering:
1. The need for a compact index dealign with multiple columns.
2. The occasional false positives in  search results, with the assurance there are no false negatives.
3. Faster and more space-efficient alternative to traditional multicolumn indexes.

# Example use case.

Agricultural enterprise: A large-scale farming operation has a database keeping track of readings from sensors distributed across many fields. These sensors monitor different parameters, such as soil moisture, sunlight intensity, and pest activity. Using a `bloom` index, the farm's database administrators can create a compact index that includes all the columns the researchers frequently query against, speeding up query times and conserving storage space.

# Example test script.

```
-- Create a sample table with pH_level as integer (assuming you want 2 decimal places precision)
CREATE TABLE farm_sensor_data AS
   SELECT
     (random() * 100)::int as soil_moisture,
     (random() * 40)::int as temperature,
     (random() * 1000)::int as sunlight_intensity,
     (round(random() * 14 * 100))::int as pH_level_100, -- 2 decimal places precision
     (random() * 100)::int as nutrient_level,
     (random() * 100)::int as pest_activity
   FROM
     generate_series(1,1000000);

-- Create a bloom index on the sensor readings table
CREATE INDEX farm_data_bloomidx ON farm_sensor_data USING bloom (soil_moisture, temperature, sunlight_intensity, pH_level_100, nutrient_level, pest_activity);

**EXPLAIN ANALYZE SELECT * FROM farm_sensor_data WHERE soil_moisture BETWEEN 40 AND 50 AND temperature > 35;
                                                     QUERY PLAN
--------------------------------------------------------------------------------------------------------------------
 Seq Scan on farm_sensor_data  (cost=0.00..2387.00 rows=1319 width=24) (actual time=0.010..8.012 rows=1292 loops=1)
   Filter: ((soil_moisture >= 40) AND (soil_moisture <= 50) AND (temperature > 35))
   Rows Removed by Filter: 98708
 Planning Time: 1.119 ms
 Execution Time: 8.062 ms
(5 rows)**

-- Drop table(s) and affiliate items to complete example
DROP TABLE farm_sensor_data CASCADE;
```
