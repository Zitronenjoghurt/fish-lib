-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fish_fishes
(
    id            BIGSERIAL PRIMARY KEY,
    user_id       BIGINT      NOT NULL REFERENCES fish_users (id),
    data_id       INTEGER     NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    size_baby_mm  REAL        NOT NULL,
    size_adult_mm REAL        NOT NULL,
    lifespan_days REAL        NOT NULL
);