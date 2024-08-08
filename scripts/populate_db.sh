#!/bin/bash

sqlite3 actors.db <<EOF
CREATE TABLE temp_actors(id INT, full_name TEXT, birth_year INT);
.mode csv
.separator ","
.headers on
.import data/new_large/actors.csv temp_actors
INSERT INTO actors (full_name, birth_year, id)
SELECT full_name, CASE WHEN birth_year = '' THEN NULL ELSE birth_year END, id FROM temp_actors;
DROP TABLE temp_actors;
EOF
