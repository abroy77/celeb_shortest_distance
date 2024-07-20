#!/bin/bash

sqlite3 actors.db <<EOF
CREATE TABLE temp_actors(id INT, full_name TEXT, birth_year INT);
.mode csv
.separator ","
.headers on
.import data/new_large/actors.csv temp_actors
INSERT INTO actors (full_name, birth_year) SELECT full_name, birth_year FROM temp_actors;
EOF
