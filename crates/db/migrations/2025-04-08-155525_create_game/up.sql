-- Your SQL goes here
CREATE TABLE Game (
    id UUID PRIMARY KEY,
    simulation_id UUID NOT NULL REFERENCES Simulation (id) ON DELETE CASCADE,
    home_team_id UUID NOT NULL REFERENCES Team (id) ON DELETE CASCADE,
    guest_team_id UUID NOT NULL REFERENCES Team (id) ON DELETE CASCADE,
    round BIGINT NOT NULL
);