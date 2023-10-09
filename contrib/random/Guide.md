# What is this extension?
> *Generate random data.*

The `random` extension for PostgreSQL introduces a suite of functions specifically designed to generate random values across various data types.

# When should you use it?

This extension may be deployed when you need to produce random data values within PostgreSQL. The use cases might include, but are not limited to generating sample data for application development or database stress testing.

# Example use case.

**Generate test data for development:**

A software developer is building a tool that can analyze product listings and suggest suitable categories based on patterns in the data. To emulate a realistic e-commerce environment, the developer uses the `random` extension to populate the database with mock products.

# Example test script.

```
-- Create example table, products, and populate with test data
CREATE TABLE products (
    product_id serial PRIMARY KEY,
    product_name text,
    description text,
    price double precision
);

DO
$$
DECLARE 
    counter INTEGER := 1;
BEGIN
    WHILE counter <= 1000 LOOP
        INSERT INTO products (product_name, description, price) VALUES
        (random_string(10), random_string(50), ROUND(random_double_precision(1, 1000)::numeric, 2));
        counter := counter + 1;
    END LOOP;
END
$$;

-- Print the first 5 rows from the products table to showcase randomized data
SELECT * FROM products LIMIT 5;

product_id | product_name |                         description                          | price
------------+--------------+--------------------------------------------------------------+--------
          1 | Q\r<"g  swD_ | LHxt*`-=%;V(.JP|#n|{3\-Y);fU8Kva(T+                         +| 643.61
            |              | YEu.]~J)KxMY$                                                |
          2 | ri1Wp:|wEW   | piYQkBQiJD\;$]sN_       ]zv1.=69pVOU    43f3%.aF;|s@12M\rD<G |   8.42
          3 | lpQ5Vev@0s   | %hF[&?d7TKQHl!?^dO1ZD%&-A_[e`?x!Z3SwbWMVPL:2cC[8*I           | 972.78
          4 | pc5OGOp^Tl   | 5zF$LGTO:\.;zD<'6_U\@!R[7{vFUya8gGDT$=QfsH/a@;\[lH           |  97.19
          5 | %G0m^&,>c`   | qd5[,RA6=*v\r7k ;WNrqD\f'2-jV%m N)nu#5d;>De?U*Q"+'d          |  62.07
(5 rows)

-- Drop table(s) and affiliate items to complete example
DROP TABLE products CASCADE ;
```
