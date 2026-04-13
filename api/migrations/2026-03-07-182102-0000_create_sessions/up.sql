-- Your SQL goes here
CREATE TABLE sessions
(
    id           UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    uid          UUID NOT NULL REFERENCES "user" (id) ON DELETE CASCADE,
    token_hash   TEXT        NOT NULL UNIQUE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at   TIMESTAMPTZ NOT NULL,
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at   TIMESTAMPTZ NULL,
    user_agent   TEXT NULL,
    ip_address   TEXT NULL
);

CREATE INDEX idx_sessions_uid ON sessions(uid);