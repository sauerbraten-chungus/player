-- Add migration script here
SELECT setval(pg_get_serial_sequence('players', 'id'), (SELECT COALESCE(MAX(id), 0) + 1 FROM players));
