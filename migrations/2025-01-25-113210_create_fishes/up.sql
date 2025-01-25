-- Your SQL goes here
CREATE TABLE fish_fishes
(
    id      BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES fish_users (id)
);