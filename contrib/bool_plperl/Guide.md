# What is this extension?
>*Transform boolean type <> PL/Perl.*

`bool_plperl` is a C-based transformation extension that allows conversion between PostgreSQL- and Perl-native boolean values.

# When should you use it?

`bool_plperl` should be considered when writing or using PL/Perl functions within PostgreSQL that involve boolean values. This extension ensures that boolean values from PostgreSQL are properly interpreted in Perl and vice versa, eliminating potential logical errors.

# Example use case.

Library:

A public library wants to check if a particular book is available for borrowing by means of a PL/Perl function. Due to the boolean value interpretation differences between PostgreSQL and Perl, they use the `bool_plperl` extension to make this process seamless.

# Example test script.

```
-- Sample table for library books
CREATE TABLE library_books (
    book_id serial PRIMARY KEY,
    title text,
    is_available bool
);
-- CREATE TABLE

-- Populate the table with a sample book
INSERT INTO library_books (title, is_available) VALUES 
('The Great Gatsby', true),
('To Kill a Mockingbird', false),
('Brothers Grimm Fairy Tales', true);
-- INSERT 0 3

-- Create a PL/Perl function to check book availability
CREATE FUNCTION is_book_available(book_title text) RETURNS bool
TRANSFORM FOR TYPE bool
AS $$
  my ($book_title) = @_;
  my $rv = spi_exec_query("SELECT is_available FROM library_books WHERE title = '$book_title'");
  return $rv->{rows}[0]->{is_available};
$$ LANGUAGE plperl;
-- CREATE FUNCTION

-- Test the function
SELECT title, is_book_available(title) as available_status FROM library_books WHERE title = 'The Great Gatsby';
title       | available_status
------------------+------------------
 The Great Gatsby | t
(1 row)
```
