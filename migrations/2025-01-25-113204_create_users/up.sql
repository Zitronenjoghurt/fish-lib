-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fish_users
(
    id          BIGSERIAL PRIMARY KEY,
    external_id BIGINT NOT NULL UNIQUE
)