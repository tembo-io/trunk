# What is this extension?

>*Custom aggregate functions.*

**`argm`** is a C-based extension that provides a set of custom aggregate functions to perform operations on database columns.


# When should you use it?

This extension is designed for data analytics, for example when interested in performing calculations across grouped data.


# Example use case.

User Login Analysis: 

A team is managing a platform where user engagement metrics, such as weekly logins, are crucial to assess the health of the product. Recently, regional differences in user activity have become an interest. Using the **``argm``** extension, the team can efficiently group users by region, compute average weekly logins, and pinpoint the most active user from each region. These insights can be leveraged to understand regional engagement patterns, optimize product features tailored to specific regions, and enhance targeted outreach strategies.


# Example test script.

```
-- Create example table, users, and populate with test data 
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    email VARCHAR(100) UNIQUE,
    ip_address VARCHAR(20),
    region VARCHAR(50),
    logins_per_week INT
);

INSERT INTO users (first_name, last_name, email, ip_address, region, logins_per_week)
VALUES
    ('Haily', 'Morgan', 'hmorgan0@prlog.org', '74.236.202.243', 'North', 57),
    ('Odo', 'Robers', 'orobers1@smugmug.com', '93.51.62.217', 'East', 54),
    ('Sammy', 'Ferrillo', 'sferrillo2@sohu.com', '168.155.137.155', 'West', 77),
    ('Sammy', 'Orcott', 'sorcott3@mit.edu', '104.91.250.13', 'South', 69),
    ('Joyann', 'Parzizek', 'jparzizek4@etsy.com', '193.244.60.165', 'East', 69),
    ('Ignacio', 'Nazer', 'inazer5@ocn.ne.jp', '211.127.59.167', 'North', 67),
    ('Lise', 'Read', 'lread6@google.ru', '53.242.22.68', 'West', 96),
    ('Granny', 'Uccelli', 'guccelli7@surveymonkey.com', '7.63.247.26', 'South', 24),
    ('Sammy', 'Barrett', 'sbarrett8@abc.net', '88.44.33.22', 'West', 50);

-- Leverage AVG to average the amount of logins per week per region, and the function argmax to showcase the email of the user with highest logins of a given region.
SELECT region, 
       AVG(logins_per_week) AS average_logins_per_week,
       argmax(email, logins_per_week) AS email_with_max_logins,
       argmax(first_name, logins_per_week) AS user_with_max_logins
FROM users
GROUP BY region;

region | average_logins_per_week | email_with_max_logins | user_with_max_logins
--------+-------------------------+-----------------------+----------------------
 East   |     61.5000000000000000 | jparzizek4@etsy.com   | Joyann
 South  |     46.5000000000000000 | sorcott3@mit.edu      | Sammy
 West   |     74.3333333333333333 | lread6@google.ru      | Lise
 North  |     62.0000000000000000 | inazer5@ocn.ne.jp     | Ignacio
(4 rows)

-- Drop table(s) to complete example
DROP TABLE users;
```
