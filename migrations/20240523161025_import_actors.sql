-- Add migration script here
-- Create a temporary table
-- CREATE TEMP TABLE temp_actors (id INT, full_name TEXT, birth_year INT);

-- Copy data into the temporary table
-- COPY temp_actors
copy actors (id, full_name, birth_year)
FROM '/mnt/actors.csv' 
DELIMITER ',' 
CSV HEADER;

-- Insert data from the temporary table into the actors table
-- INSERT INTO actors (id, full_name, birth_year)
-- SELECT full_name, birth_year FROM temp_actors;

-- -- Drop the temporary table
-- DROP TABLE temp_actors;