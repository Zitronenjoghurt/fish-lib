-- Your SQL goes here
CREATE TABLE fishes
(
    id      BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users (id)
);