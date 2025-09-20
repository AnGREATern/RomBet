-- Your SQL goes here
CREATE TABLE Bet (
    id TEXT NOT NULL PRIMARY KEY,
    simulation_id TEXT NOT NULL REFERENCES Simulation (id) ON DELETE CASCADE,
    amount BIGINT NOT NULL,
    coefficient INTEGER NOT NULL,
    game_id TEXT NOT NULL REFERENCES Game (id) ON DELETE CASCADE,
    event BLOB NOT NULL,
    is_won BOOLEAN
);