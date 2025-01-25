-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fish_fishes
(
    id      BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES fish_users (id)
);