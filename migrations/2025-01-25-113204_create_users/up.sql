-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fish_users
(
    id          BIGSERIAL PRIMARY KEY,
    external_id BIGINT      NOT NULL UNIQUE,
    credits     BIGINT      NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    timezone    VARCHAR     NOT NULL DEFAULT 'UTC'
)