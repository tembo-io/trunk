# What is this extension?
> *Array-specific aggregate functions.*

`aggs_for_vecs` is a C-based extension that introduces arithmetic operations on arrays, as opposed to scalars. In the case of `aggs_for_vecs`, each array is treated as a vector.


# When should you use it?
This extension can offer element-wise operations across multiple rows, which could be useful when performing calculations on specific array positions, e.g., finding the minimum value across multiple rows for each element in the array.


# Example use case.

Customer Behavior Analysis:

An e-commerce platform is using rating arrays to capture multi-faceted feedback on products. Each element in the array represents a specific aspect, i.e., usability, design, performance, and customer support. By analyzing these ratings over time, this business can gain insights into product trends and customer satisfaction. Different sources, such as review sites or direct feedback, contribute to these ratings. This aggregated data offers a comprehensive view of a product's performance, guiding improvements and strategies.


# Example test script.
```
-- Create example table, rating_aspects, and populate with test data
CREATE TABLE rating_aspects (
    aspect_id SERIAL PRIMARY KEY,
    aspect_name VARCHAR(255)
);

INSERT INTO rating_aspects(aspect_name) VALUES
('Usability'),
('Design'),
('Performance'),
('Customer Support');

-- Create example table, product_ratings, and populate with test data
-- The ratings array represents scores for aspects in the order: [Usability, Design, Performance, Customer Support]
CREATE TABLE product_ratings (
    id SERIAL PRIMARY KEY,
    date DATE,
    ratings FLOAT[],
    source VARCHAR(255)
);

INSERT INTO product_ratings(date, ratings, source) VALUES
('2023-01-01', '{3.5,4.2,4.0,4.5}', 'Source A'),
('2023-01-01', '{4.0,3.8,4.1,4.2}', 'Source B'),
('2023-01-01', '{3.8,4.0,3.9,4.3}', 'Source C'),
('2023-01-01', '{3.9,4.1,4.2,4.4}', 'Source D'),
('2023-01-01', '{3.7,4.0,4.1,4.0}', 'Source E'),
('2023-01-02', '{4.0,4.2,3.9,4.5}', 'Source A'),
('2023-01-02', '{3.8,4.0,4.2,4.1}', 'Source B'),
('2023-01-02', '{4.0,4.1,3.8,4.3}', 'Source C'),
('2023-01-02', '{3.9,4.3,4.0,4.2}', 'Source D'),
('2023-01-02', '{3.7,4.1,4.1,4.1}', 'Source E'),
('2023-01-03', '{4.2,4.1,4.0,4.6}', 'Source A'),
('2023-01-03', '{3.9,4.0,3.9,4.1}', 'Source B'),
('2023-01-03', '{4.1,4.1,3.7,4.3}', 'Source C'),
('2023-01-03', '{4.0,4.2,4.1,4.3}', 'Source D'),
('2023-01-03', '{3.8,4.0,3.9,4.0}', 'Source E');

-- Leverage the function vec_to_mean in order to investigate the the average rating for each aspect each day
SELECT 
    source,
    unnest(array['Usability', 'Design', 'Performance', 'Customer Support']) as aspect,
    unnest(vec_to_mean(ratings)) AS avg_rating
FROM product_ratings
GROUP BY source
ORDER BY source, aspect;

source  |      aspect      |     avg_rating
----------+------------------+--------------------
 Source A | Customer Support |  4.533333333333333
 Source A | Design           |  4.166666666666667
 Source A | Performance      |  3.966666666666667
 Source A | Usability        |                3.9
 Source B | Customer Support |  4.133333333333334
 Source B | Design           |  3.933333333333333
 Source B | Performance      |  4.066666666666666
 Source B | Usability        |                3.9
 Source C | Customer Support |                4.3
 Source C | Design           |  4.066666666666666
 Source C | Performance      |                3.8
 Source C | Usability        | 3.9666666666666663
 Source D | Customer Support |                4.3
 Source D | Design           |                4.2
 Source D | Performance      | 4.1000000000000005
 Source D | Usability        | 3.9333333333333336
 Source E | Customer Support |  4.033333333333333
 Source E | Design           |  4.033333333333333
 Source E | Performance      |  4.033333333333333
 Source E | Usability        | 3.7333333333333334
(20 rows)

-- Leverage the functions vec_to_min and vec_to_max in order to investigate the minimum and maximum ratings across sources for each aspect on each day
WITH MinRatings AS (
    SELECT 
        date,
        unnest(array['Usability', 'Design', 'Performance', 'Customer Support']) as aspect,
        unnest(vec_to_min(ratings)) AS min_rating
    FROM product_ratings
    GROUP BY date
),

MaxRatings AS (
    SELECT 
        date,
        unnest(array['Usability', 'Design', 'Performance', 'Customer Support']) as aspect,
        unnest(vec_to_max(ratings)) AS max_rating
    FROM product_ratings
    GROUP BY date
)

SELECT
    MinRatings.date,
    MinRatings.aspect,
    MinRatings.min_rating,
    MaxRatings.max_rating
FROM MinRatings
JOIN MaxRatings ON MinRatings.date = MaxRatings.date AND MinRatings.aspect = MaxRatings.aspect
ORDER BY MinRatings.date, MinRatings.aspect;

date    |      aspect      | min_rating | max_rating
------------+------------------+------------+------------
 2023-01-01 | Customer Support |          4 |        4.5
 2023-01-01 | Design           |        3.8 |        4.2
 2023-01-01 | Performance      |        3.9 |        4.2
 2023-01-01 | Usability        |        3.5 |          4
 2023-01-02 | Customer Support |        4.1 |        4.5
 2023-01-02 | Design           |          4 |        4.3
 2023-01-02 | Performance      |        3.8 |        4.2
 2023-01-02 | Usability        |        3.7 |          4
 2023-01-03 | Customer Support |          4 |        4.6
 2023-01-03 | Design           |          4 |        4.2
 2023-01-03 | Performance      |        3.7 |        4.1
 2023-01-03 | Usability        |        3.8 |        4.2
(12 rows)

-- Drop table(s) to complete example
DROP TABLE rating_aspects ;
DROP TABLE product_ratings ;
```
