-- Remove the steam_id column (this is safe as long as no foreign keys or indexes depend on it)
ALTER TABLE players DROP COLUMN steam_id;

-- Add new columns. Assuming:
-- - frags: integer (e.g., number of kills in a game, NOT NULL with default 0 for existing rows)
-- - deaths: integer (e.g., number of deaths, NOT NULL with default 0)
-- - accuracy: float (e.g., percentage like 75.5, NULLable if not always available; adjust if you prefer DECIMAL for precision)
ALTER TABLE players ADD COLUMN frags INT NOT NULL DEFAULT 0;
ALTER TABLE players ADD COLUMN deaths INT NOT NULL DEFAULT 0;
ALTER TABLE players ADD COLUMN accuracy INT NOT NULL DEFAULT 0;

-- Optional: If you want to update existing rows with sample data (e.g., for the row you inserted in the first migration)
UPDATE players SET frags = 100, deaths = 50, accuracy = 75 WHERE id = 1;
