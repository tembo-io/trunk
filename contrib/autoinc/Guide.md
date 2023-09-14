# What is this extension?
>*Auto-incrementing field functions.*

`autoinc` is a C-based extension that is included in the PostgreSQL additional supplied module catalog. It provides a trigger named **autoinc()**, which helps in automatically incrementing integer fields.

# When should you use it?

This extension proves beneficial when one is concerned with auto-incrementing fields under specific conditions, for example when only certain rows or entries require incrementing.


# Example use case.

Limited release cataloging:

A fountain pen manufacturing company occasionally releases limited edition pens under specific design series themes. Each design series has a distinct name and theme, with every pen in it receiving a unique ID, starting from 1. By utilizing the **`autoinc`** trigger, this firm can automatically allocate a distinct ID to each pen within its design series.


# Example test script.
```
-- Create sequence for each design series
-- Starting from 1 for Galaxy
CREATE SEQUENCE galaxy_seq START 1;

-- Starting from 1 for Nature
CREATE SEQUENCE nature_seq START 1;

-- Starting from 1 for Random
CREATE SEQUENCE random_seq START 1;

-- Create example table, limited_edition_pens
CREATE TABLE limited_edition_pens (
    design_series     text,
    pen_id            int4,
    model             text,
    paired_ink_color  text,
    year_of_release   int4,
    PRIMARY KEY (design_series, pen_id)
);

-- Create trigger for each series to auto-increment using the respective sequence
CREATE TRIGGER lep_galaxy_autoinc
    BEFORE INSERT ON limited_edition_pens
    FOR EACH ROW WHEN (NEW.design_series = 'Galaxy')
    EXECUTE PROCEDURE autoinc(pen_id, galaxy_seq);

CREATE TRIGGER lep_nature_autoinc
    BEFORE INSERT ON limited_edition_pens
    FOR EACH ROW WHEN (NEW.design_series = 'Nature')
    EXECUTE PROCEDURE autoinc(pen_id, nature_seq);

CREATE TRIGGER lep_random_autoinc
    BEFORE INSERT ON limited_edition_pens
    FOR EACH ROW WHEN (NEW.design_series = 'Random')
    EXECUTE PROCEDURE autoinc(pen_id, random_seq);

-- Now, you can insert data
INSERT INTO limited_edition_pens(design_series, model, paired_ink_color, year_of_release) VALUES ('Galaxy', 'Stellar', 'Martian Mud Red', 2022);
INSERT INTO limited_edition_pens(design_series, model, paired_ink_color, year_of_release) VALUES ('Galaxy', 'Nebula', 'Comet Tail Blue', 2022);
INSERT INTO limited_edition_pens(design_series, model, paired_ink_color, year_of_release) VALUES ('Nature', 'Woodland', 'Gnome Forest Green', 2023);
INSERT INTO limited_edition_pens(design_series, model, paired_ink_color, year_of_release) VALUES ('Nature', 'Ocean', 'Coastal Sunrise Orange', 2023);
INSERT INTO limited_edition_pens(design_series, model, paired_ink_color, year_of_release) VALUES ('Nature', 'Mountain', 'Twin-Peak Purple', 2023);
INSERT INTO limited_edition_pens(design_series, model, paired_ink_color, year_of_release) VALUES ('Random', 'Twister', 'Galaboot Blue', 2023);
INSERT INTO limited_edition_pens(design_series, model, paired_ink_color, year_of_release) VALUES ('Random', 'Mission24', 'Ironton Silver', 2023);

-- Verify that the pen IDs increment as expected
SELECT * FROM limited_edition_pens ORDER BY design_series, pen_id;

-- Drop table(s) and sequence(s) to complete example
DROP TABLE limited_edition_pens;
DROP SEQUENCE galaxy_seq;
DROP SEQUENCE nature_seq;
DROP SEQUENCE random_seq;
```
