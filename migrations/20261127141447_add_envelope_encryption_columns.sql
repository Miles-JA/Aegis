-- Add migration script here
ALTER TABLE secrets
ADD COLUMN encrypted_dek BYTEA NOT NULL,
ADD COLUMN dek_nonce BYTEA NOT NULL
    CHECK (octet_length(dek_nonce) = 12);