# What is this extension?

> Cube data type.

The `cube` extension provides a data type for representing multidimensional cubes alongside a set of functions and operators for working with these cubes.

# When should you use it?

This extension can be used when considering multi-dimensional data, for example, coordinates in a 3D space or multi-dimensional metrics. 

# Example use case.

Suppose a research group is interested in studying the movements of a new fox population spotted in the vicinity of Angel Rocks Trailhead, near Fairbanks, Alaska.  Select foxes are equipped with GPS trackers that record their locations at regular intervals. Each data point contains 3D coordinates representing latitude, longitude, and altitude.

Utilizing the `cube` extension, this data is be stored and organized efficiently within a PostgreSQL database. The research group can then run queries to analyze various aspects of fox movements, identify territorial boundaries, and observe proximity between different foxes over time.

# Example test script.

```
-- Create example table, fox_locations
CREATE TABLE fox_locations (
    fox_id INT,
    location cube,
    timestamp TIMESTAMP
);

-- Create function to simulate random movement of foxes
CREATE OR REPLACE FUNCTION random_movement(
    base_lat DOUBLE PRECISION,
    base_long DOUBLE PRECISION,
    base_alt DOUBLE PRECISION
)
RETURNS cube AS $$
DECLARE
    new_lat DOUBLE PRECISION;
    new_long DOUBLE PRECISION;
    new_alt DOUBLE PRECISION;
BEGIN
    -- Random variation in latitude, longitude, and altitude
    new_lat := base_lat + (random() - 0.5) / 100;
    new_long := base_long + (random() - 0.5) / 100;
    new_alt := base_alt + (random() - 0.5) * 20;
    
    -- Return new coordinates as a cube
    RETURN cube(array[new_lat, new_long, new_alt]);
END;
$$ LANGUAGE plpgsql;

-- Populate with test data of 5 foxes over 7 days at 8 hour intervals
DO $$
DECLARE 
    day INT;
    hour INT;
    fox INT;
BEGIN
    FOR day IN 1..7 LOOP
        FOR hour IN 1..3 LOOP
            FOR fox IN 1..5 LOOP
                INSERT INTO fox_locations (fox_id, location, timestamp)
                VALUES (
                    fox,
                    random_movement(65.011915, -146.216302, 130.0), -- Assuming foxes are moving around the Angel Rocks Trailhead
                    timestamp '2023-10-12 00:00:00' + interval '1 day' * day + interval '8 hours' * hour
                );
            END LOOP;
        END LOOP;
    END LOOP;
END;
$$;

-- Create an index on the location column to speed up geometric queries
CREATE INDEX idx_fox_locations_location ON fox_locations USING gist (location);

-- Query to find foxes that were within a 0.5 degree cube of point (65.011915, -146.216302, 130.0) at any time
SELECT fox_id, location, timestamp
FROM fox_locations
WHERE location <@ cube_enlarge(cube(array[65.011915, -146.216302, 130.0]), 0.5, 3);

-- Drop table(s) and affiliate items to complete example
DROP TABLE fox_locations CASCADE;
```
