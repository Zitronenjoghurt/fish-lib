-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fish_ponds
(
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT      NOT NULL REFERENCES fish_users (id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    capacity   INTEGER     NOT NULL
);