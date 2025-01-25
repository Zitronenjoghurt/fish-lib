-- Your SQL goes here
CREATE TABLE fish_users
(
    id          BIGSERIAL PRIMARY KEY,
    external_id BIGINT NOT NULL UNIQUE
)