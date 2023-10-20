# What is this extension?

> Define authorization protocols.

`credcheck` is designed to enforce credential policies within a database system. It provides various checks on username and password attributes to enhance security and compliance. By defining rules for credential characteristics such as minimum length, character type inclusion and exclusion, repetition of characters, and password expiration, database administrators can ensure that user credentials adhere to the desired policies.

# When should you use it?

This extension can be used to implement specific rules on username and password attributes, relating, but not limited to to character count, special character usage, and password reuse.

# Example use case.

A country’s National Park Authority manages a PostgreSQL database to store and process data relating to wildlife sightings, visitor statistics, and park maintenance. To ensure data integrity and security, they decide to utilize the `credcheck` extension to enforce credential policies. This way, they can ensure that only personnel with strong, compliant credentials can access the database, thereby reducing the risk of unauthorized access or data manipulation.

# Example test script.

```
-- Configure credential policies to enforce username and password standards and reload configuration files
ALTER SYSTEM SET credcheck.username_min_length = 4;
ALTER SYSTEM SET credcheck.username_min_special = 1;
ALTER SYSTEM SET credcheck.password_min_length = 8;
ALTER SYSTEM SET credcheck.password_min_special = 1;
SELECT pg_reload_conf();

-- Attempt to create a user for a new park ranger, which does not meet the credential policies
CREATE USER ranger_ WITH PASSWORD 'forest';
ERROR:  password length should match the configured credcheck.password_min_length

-- Correct the mistake and create a user for the new park ranger with compliant credentials
CREATE USER ranger_ WITH PASSWORD 'forest$2023';
CREATE ROLE

-- Now, an administrative staff tries to create an account but misses the special character in username
CREATE USER adminstaff WITH PASSWORD 'Admin$2023';
ERROR:  username does not contain the configured credcheck.username_min_special characters

-- Correct the mistake and create a user for the administrative staff with compliant credentials
CREATE USER admin$staff WITH PASSWORD 'Admin$2023';
CREATE ROLE

-- DROP users
DROP USER ranger_;
DROP USER admin$staff;

-- Spotlight current username and password standards
SHOW credcheck.username_min_length;
SHOW credcheck.username_min_special;
SHOW credcheck.password_min_length;
SHOW credcheck.password_min_special;

-- Reset select parameters
ALTER SYSTEM RESET credcheck.username_min_length;
ALTER SYSTEM RESET credcheck.username_min_special;
ALTER SYSTEM RESET credcheck.password_min_length;
ALTER SYSTEM RESET credcheck.password_min_special;
SELECT pg_reload_conf();

-- Confirm changes to end the example
SHOW credcheck.username_min_length;
SHOW credcheck.username_min_special;
SHOW credcheck.password_min_length;
SHOW credcheck.password_min_special;
```
