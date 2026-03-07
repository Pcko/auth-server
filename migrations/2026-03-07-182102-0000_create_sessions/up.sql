-- Your SQL goes here
CREATE TABLE sessions
(
    id           UUID PRIMARY KEY,
    uid          UUID PRIMARY KEY,
    token_hash   TEXT        NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at   TIMESTAMPTZ NOT NULL,
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at   TIMESTAMPTZ,
    user_agent   TEXT,
    ip_address   TEXT
);