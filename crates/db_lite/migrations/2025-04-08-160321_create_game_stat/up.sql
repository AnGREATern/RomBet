-- Your SQL goes here
CREATE TABLE GameStat (
    id TEXT NOT NULL PRIMARY KEY,
    game_id TEXT NOT NULL REFERENCES Game (id) ON DELETE CASCADE,
    home_team_total SMALLINT NOT NULL,
    guest_team_total SMALLINT NOT NULL
);