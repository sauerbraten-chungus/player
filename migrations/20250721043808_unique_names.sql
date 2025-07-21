-- Add migration script here
ALTER TABLE players ADD CONSTRAINT unique_name UNIQUE (name);
