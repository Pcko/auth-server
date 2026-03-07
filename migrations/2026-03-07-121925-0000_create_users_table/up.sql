CREATE TABLE "user"
(
    id             UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    email          TEXT        NOT NULL UNIQUE,
    name           TEXT        NOT NULL,
    password_hash  TEXT        NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_allowed     BOOLEAN     NOT NULL DEFAULT TRUE,
    is_mfa_enabled BOOLEAN     NOT NULL DEFAULT FALSE
);