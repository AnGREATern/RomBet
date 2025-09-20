-- Your SQL goes here
CREATE TABLE Game (
    id TEXT NOT NULL PRIMARY KEY,
    simulation_id TEXT NOT NULL REFERENCES Simulation (id) ON DELETE CASCADE,
    home_team_id TEXT NOT NULL REFERENCES Team (id) ON DELETE CASCADE,
    guest_team_id TEXT NOT NULL REFERENCES Team (id) ON DELETE CASCADE,
    round BIGINT NOT NULL
);