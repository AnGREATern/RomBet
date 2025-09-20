-- Your SQL goes here
CREATE TABLE Simulation (
    id TEXT NOT NULL PRIMARY KEY,
    ip TEXT UNIQUE NOT NULL,
    round BIGINT NOT NULL,
    balance BIGINT NOT NULL
);