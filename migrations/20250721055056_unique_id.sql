-- Add migration script here
ALTER TABLE players
    ALTER COLUMN id SET DATA TYPE BIGINT, -- Optional: Use BIGINT for larger capacity
    ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
        START WITH 1
        INCREMENT BY 1
    );
