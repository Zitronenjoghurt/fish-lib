-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fish_specimens
(
    id                  BIGSERIAL PRIMARY KEY,
    user_id             BIGINT      NOT NULL REFERENCES fish_users (id),
    species_id          INTEGER     NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    size_baby_ratio     REAL        NOT NULL,
    size_adult_ratio    REAL        NOT NULL,
    lifespan_days_ratio REAL        NOT NULL,
    catch_age           REAL        NOT NULL
);