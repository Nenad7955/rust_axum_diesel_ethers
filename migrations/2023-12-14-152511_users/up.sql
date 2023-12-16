-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Users (
                                     id VARCHAR PRIMARY KEY,
                                     username VARCHAR NOT NULL UNIQUE,
                                     pubkey VARCHAR NOT NULL UNIQUE
)