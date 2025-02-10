-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fish_items
(
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT      NOT NULL REFERENCES fish_users (id),
    type_id    INTEGER     NOT NULL,
    properties JSONB       NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)