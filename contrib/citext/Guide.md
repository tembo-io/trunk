# What is this extension?

> *Match case-insensitive text.*

The `citext` extension provides a case-insensitive character string type, called `CITEXT`. While it behaves almost exactly like the `TEXT` type in PostgreSQL, comparisons against a `CITEXT` type are case-insensitive.

# When should you use it?

This extension can be used when you want case-insensitive behavior without having to change your queries or rely on functions like `LOWER()`. It's particularly useful for columns containing data that's often queried by end-users, such as usernames, email addresses, or other identifiers where the capitalization might vary.  This behavior allows for a more forgiving query experience, particularly useful when handling data that might have inconsistent capitalizations.

# Example use case.

Consider a coffee shop that offers a digital menu where customers can search for their favorite coffee by region, country, or bean type. As customers search for their preferred beans, they might type "Arabica", "arabica", "ARABICA", or, though less likely, "aRaBiCa". Similarly, when searching by region or country, variations like "Columbia" or "columbia" might be entered. The owner doesn't want the search to miss any items just because of case discrepancies. To address this, the database storing the coffee varieties uses the `CITEXT` data type for columns such as bean_type, region, and country. This ensures that no matter how a customer capitalizes their search, they will always get the right results.

# Example test script.

```
-- Create example table, coffee_varieties, and populate with test data
CREATE TABLE coffee_varieties (
    id SERIAL PRIMARY KEY,
    bean_name CITEXT NOT NULL,
    bean_type CITEXT CHECK(bean_type IN ('Arabica', 'Robusta')),
    region CITEXT NOT NULL,
    country CITEXT NOT NULL,
    description TEXT
);

INSERT INTO coffee_varieties (bean_name, bean_type, region, country, description) VALUES
    ('Golden Colombian', 'Arabica', 'South America', 'Colombia', 'A rich coffee with fruity tones.'),
    ('Viet Delight', 'Robusta', 'Asia', 'Vietnam', 'Strong and bold with a hint of chocolate.'),
    ('Ethiopian Dawn', 'Arabica', 'Africa', 'Ethiopia', 'Earthy flavor with wine-like subtlety.'),
    ('Java Roast', 'Arabica', 'Asia', 'Indonesia', 'Dark and strong, perfect for espresso.'),
    ('African Robust', 'Robusta', 'Africa', 'Uganda', 'Full-bodied with a nutty flavor.');

-- A customer searches for coffee from Colombia without considering capitalization
SELECT * FROM coffee_varieties WHERE country = 'colombiA';
 id |     bean_name     | bean_type |    region    | country |                 description                 
----+-------------------+-----------+--------------+---------+---------------------------------------------
  1 | Golden Colombian  | Arabica   | South America| Colombia| A rich coffee with fruity tones.
(1 row)

-- Another customer is interested in any "arabica" beans, again not worrying about capitalization
SELECT * FROM coffee_varieties WHERE bean_type = 'ARABICA';
 id |     bean_name     | bean_type |    region    | country |                 description                 
----+-------------------+-----------+--------------+---------+---------------------------------------------
  1 | Golden Colombian  | Arabica   | South America| Colombia| A rich coffee with fruity tones.
  3 | Ethiopian Dawn    | Arabica   | Africa       | Ethiopia| Earthy flavor with wine-like subtlety.
  4 | Java Roast        | Arabica   | Asia         | Indonesia| Dark and strong, perfect for espresso.
(3 rows)

--DROP tables(s) and affiliate items to complete example
DROP TABLE coffee_varieties CASCADE ;
```
