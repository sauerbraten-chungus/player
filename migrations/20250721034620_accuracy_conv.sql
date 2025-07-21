-- Add migration script here
ALTER TABLE players ALTER COLUMN accuracy type FLOAT;
UPDATE players SET accuracy = accuracy + matches_played;
