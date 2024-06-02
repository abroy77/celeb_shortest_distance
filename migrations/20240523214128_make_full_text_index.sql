-- Add migration script here
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX idx_full_name ON actors USING GIN (full_name gin_trgm_ops);