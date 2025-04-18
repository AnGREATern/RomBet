-- Your SQL goes here
CREATE TABLE GameStat (
    id UUID PRIMARY KEY,
    game_id UUID NOT NULL REFERENCES Game (id) ON DELETE CASCADE,
    home_team_total SMALLINT NOT NULL,
    guest_team_total SMALLINT NOT NULL
);