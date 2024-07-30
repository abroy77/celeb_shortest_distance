-- Add migration script here
-- Create virtual table
CREATE VIRTUAL TABLE actors USING fts5(full_name, birth_year UNINDEXED,id UNINDEXED, tokenize="trigram", prefix="5 7 10");
 
