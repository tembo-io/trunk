# What is this extension?
>***R procedural language.***

`PL/R` is a C-based procedural language extension that allows the user to compose R programming language scripts within Postgres.

# When should you use it?

By means of procedural language capabilities, this extension grants the user the power of the R programming language, which has a well-deserved reputation in fields relating, but not limited to statistical analysis.

# Example use case.

Blood Pressure Treatment Analysis:

A medical research team is evaluating the effectiveness of two distinct blood pressure treatments. On reviewing the anonymized data on patient responses, there's an interest in statistically determining whether one treatment outperforms the other. Using **`PL/R`** , the team applies R's wilcox.test function within their Postgres database. It’s the team’s hope that the test would yield a p-value is lower than the standard 0.05 threshold, such that they can celebrate that the treatments show statistically-significant differences.

# Example test script.
```
-- Create example table, bp_reduction, and populate with test data
CREATE TABLE bp_reduction (
    patient_id serial PRIMARY KEY,
    treatment_type VARCHAR(20) NOT NULL,  -- 'TreatmentA' or 'TreatmentB'
    reduction_value FLOAT NOT NULL        -- Blood pressure reduction value
);

INSERT INTO bp_reduction (treatment_type, reduction_value) VALUES
('TreatmentA', 6.3),
('TreatmentA', 6.4),
('TreatmentA', 6.5),
('TreatmentA', 6.3),
('TreatmentA', 6.4),
('TreatmentB', 6.4),
('TreatmentB', 6.6),
('TreatmentB', 6.5),
('TreatmentB', 6.5),
('TreatmentB', 6.6);

-- Create PL/R function
CREATE OR REPLACE FUNCTION simple_mann_whitney_test(a float8[], b float8[]) RETURNS FLOAT8 AS $$
    result <- wilcox.test(a, b, alternative = "two.sided")
    return(as.numeric(result$p.value))
$$ LANGUAGE 'plr';

-- Run statistical analysis
SELECT simple_mann_whitney_test(
    ARRAY(SELECT reduction_value FROM bp_reduction WHERE treatment_type = 'TreatmentA'),
    ARRAY(SELECT reduction_value FROM bp_reduction WHERE treatment_type = 'TreatmentB')
) AS p_value;

p_value
--------------------
 0.0524116286710287
(1 row)
```
