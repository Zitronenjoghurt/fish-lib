-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fish_fishing_history_entries
(
    id                        BIGSERIAL PRIMARY KEY,
    user_id                   BIGINT      NOT NULL REFERENCES fish_users (id),
    species_id                INTEGER     NOT NULL,
    created_at                TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    caught_count              INTEGER     NOT NULL,
    sold_count                INTEGER     NOT NULL,
    smallest_catch_size_ratio REAL        NOT NULL,
    largest_catch_size_ratio  REAL        NOT NULL,
    last_catch                TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    first_sell                TIMESTAMPTZ          DEFAULT NULL,
    last_sell                 TIMESTAMPTZ          DEFAULT NULL
);