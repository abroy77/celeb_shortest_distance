-- Add migration script here

CREATE TABLE
    actors (
        id serial PRIMARY KEY,
        full_name TEXT NOT NULL,
        birth_year INT
    );