# What is this extension?

> Integer indexing control. 

The `dict_int` extension serves as an example of a dictionary template for enhancing full-text search capabilities in PostgreSQL. It aims to manage the indexing of integers efficiently to prevent a surge in unique words, which could hamper search performance.

# When should you use it?

This extension could be implemented when considering a database with a substantial amount of integer data, upon which full-text searches need to be performed.

# Example use case.

Suppose you are managing a database for a large retail store, and you have a table containing millions of product IDs. You want to enable full-text search on this table to allow users to search for products using their IDs. By employing `dict_int`, you can control how these integers (product IDs) are indexed.

# Example test script.

```
-- Create a text search configuration
CREATE TEXT SEARCH CONFIGURATION test_config (COPY = english);

-- Alter the text search configuration to use the intdict dictionary for integers
ALTER TEXT SEARCH CONFIGURATION test_config
    ALTER MAPPING FOR int, uint WITH intdict;

-- Create example table, products, and populate with test data
CREATE TABLE IF NOT EXISTS products (
    product_id serial PRIMARY KEY,
    product_name varchar(255)
);

INSERT INTO products (product_name)
VALUES 
('Product A'), ('Product B'), ('Product C'),
('Product D'), ('Product E'), ('Product F'),
('Product G'), ('Product H'), ('Product I'),
('Product J'), ('Product K'), ('Product L'),
('Product M'), ('Product N'), ('Product O'),
('Product P'), ('Product Q'), ('Product R'),
('Product S'), ('Product T'), ('Product U'),
('Product V'), ('Product W'), ('Product X'),
('Product Y'), ('Product Z');

-- Create a tsvector column to store the text search vector
ALTER TABLE products ADD COLUMN tsv tsvector;

-- Update the tsvector column with the text search vector of the product_id column
UPDATE products SET tsv = to_tsvector('test_config', product_id::text);

-- Now you can perform full-text search queries on the product_id column
-- Search for a product with product_id 10
SELECT * FROM products WHERE tsv @@ to_tsquery('test_config', '10');

-- Drop table(s) and affiliate items to complete example
DROP TABLE products CASCADE;
DROP TEXT SEARCH CONFIGURATION IF EXISTS test_config;
```
