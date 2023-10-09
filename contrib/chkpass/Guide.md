# What is this extension?

> Store encrypted passwords.

`chkpass` is C-based extension that provides encrypted password storage capabilities. Originally bundled with PostgreSQL versions earlier than 11, it was later removed, leading to the development of this standalone version for newer releases.

# When should you use it?

This extension might not be suitable for applications with high-security requirements or in cases where more standardized and robust security measures are preferred.

This extension could be used when:
- You need to store passwords or sensitive strings in a PostgreSQL database.
- You want to store these strings in an obfuscated form to prevent easy inspection and retrieval.
- You want a simplified way to check equality between an input string and the stored obfuscated string without manual decryption.

# Example use case.

A group of developer friends, enthusiastic about building a web application for fun and learning, recognizes the importance of security, especially when handling user passwords. Before they delve into more advanced security measures, they decide to integrate the `chkpass` extension into their PostgreSQL database. This provides them with a foundational level of encryption, ensuring that even at the early stages of their project, user passwords are not stored in plain text.

# Example test script.

```
-- Create a table, users
CREATE TABLE users (
    user_id serial PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password chkpass NOT NULL
);

-- A new user "tech_guru" signs up and chooses the password "secure123"
INSERT INTO users (username, password) VALUES ('tech_guru', 'secure123');

-- Displaying how the password looks after being encrypted by chkpass
SELECT username, password FROM users WHERE username = 'tech_guru';
   username   |     password     
-------------+------------------
 tech_guru   | :4jsl9D/r8B5z.
(1 row)

-- Verifying the password. This will return true if the provided password matches the stored one
SELECT password = 'secure123' AS is_correct FROM users WHERE username = 'tech_guru';
 is_correct 
------------
 t
(1 row)

-- Trying to verify with a wrong password
SELECT password = 'wrongPass' AS is_correct FROM users WHERE username = 'tech_guru';
 is_correct 
------------
 f
(1 row)

-- Drop table(s) and affiliate items to complete example
DROP TABLE users CASCADE ;
```
