-- Add migration script here
CREATE TABLE players (
    id int primary key,
    name varchar(80) not null,
    steam_id varchar(255) not null,
    matches_played int not null,
    elo int not null,
    commendations int not null,
    created_at date not null
);

INSERT INTO players (id, name, steam_id, matches_played, elo, commendations, created_at)
    VALUES (1, 'Jane Doe', '1', '25', '9001', '42', '2025-07-14');
