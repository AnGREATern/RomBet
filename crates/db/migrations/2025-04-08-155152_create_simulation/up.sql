-- Your SQL goes here
CREATE TABLE Simulation (
    id UUID PRIMARY KEY,
    ip TEXT UNIQUE NOT NULL,
    round BIGINT NOT NULL,
    balance BIGINT NOT NULL
);