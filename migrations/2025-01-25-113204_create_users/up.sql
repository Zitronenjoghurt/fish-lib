-- Your SQL goes here
CREATE TABLE users
(
    id          BIGSERIAL PRIMARY KEY,
    external_id BIGINT NOT NULL UNIQUE
)