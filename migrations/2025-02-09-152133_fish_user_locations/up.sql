-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fish_user_locations
(
    user_id     BIGINT      NOT NULL REFERENCES fish_users (id),
    location_id INTEGER     NOT NULL,
    unlocked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, location_id)
)