-- Add migration script here
CREATE TABLE IF NOT EXISTS secrets (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    ciphertext BYTEA NOT NULL,
    nonce BYTEA NOT NULL
        CHECK (octet_length(nonce) = 12)
);