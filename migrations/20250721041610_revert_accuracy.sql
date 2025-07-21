-- Add migration script here
ALTER TABLE players ALTER COLUMN accuracy TYPE INT USING ROUND(accuracy);
