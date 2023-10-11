# What is this extension?

> COUNT(DISCTINCT ...) alternative.

The COUNT_DISTINCT aggregate extension offers a better performance alternative to the traditional COUNT(DISTINCT ...). Especially for larger datasets, where the traditional method may end in slow sorts. This extension includes polymorphic aggregate functions, which can handle both fixed-length data types and arrays of these types.

# When should you use it?

This extension can be used for counting distinct values in large datasets, particularly in earlier PostgreSQL versions. However, it may not be suitable if memory consumption is a concern or if using newer PostgreSQL releases with improved COUNT(DISTINCT ...) performance.

# Example use case.

**Urban Rentals Analysis:**

In a city, local authorities aim to analyze the rental data of various mobility options across 20 districts over a year to aid in urban planning and traffic management. The data comprises different vehicles rented each month. Using a local PostgreSQL setup, they employ the COUNT_DISTINCT extension to efficiently analyze the large dataset, particularly counting the distinct monthly utilization of each rental type across all districts.

# Example test script.

```
-- Create example table, rentals, and populate with test data
CREATE TABLE rentals (
    rental_id SERIAL PRIMARY KEY,
    district INT NOT NULL,
    rental_type CITEXT NOT NULL,
    month INT NOT NULL,
    rental_count INT NOT NULL
);

INSERT INTO rentals (district, rental_type, month, rental_count)
SELECT
    district,
    rental_type,
    month,
    (100 * random())::int  -- Random rental count for variety
FROM
    generate_series(1, 20) district,
    unnest(array['bike', 'turbo-bike', 'scooter', 'turbo-scooter', 'tricycle', 'turbo-tricycle', 'unicycle', 'turbo-unicycle', 'stroller', 'turbo-stroller']) rental_type,
    generate_series(1, 12) month;

-- Analyze the rentals table to update statistics for the planner
ANALYZE rentals;

-- Example 1: Using COUNT(DISTINCT ...)
EXPLAIN ANALYZE
SELECT district, rental_type, COUNT(DISTINCT month)
FROM rentals
GROUP BY district, rental_type;

-- Example 2: Using COUNT_DISTINCT()
EXPLAIN ANALYZE
SELECT district, rental_type, COUNT_DISTINCT(month)
FROM rentals
GROUP BY district, rental_type;

-- Drop table(s) and affiliate items to complete example
DROP TABLE rentals CASCADE ;
```
