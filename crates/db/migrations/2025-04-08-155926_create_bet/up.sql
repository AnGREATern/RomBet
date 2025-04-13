-- Your SQL goes here
CREATE TABLE Bet (
    id UUID PRIMARY KEY,
    simulation_id UUID NOT NULL REFERENCES Simulation (id) ON DELETE CASCADE,
    amount BIGINT NOT NULL,
    coefficient INTEGER NOT NULL,
    game_id UUID NOT NULL REFERENCES Game (id) ON DELETE CASCADE,
    event BYTEA NOT NULL,
    is_won BOOLEAN
);