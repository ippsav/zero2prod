-- Add migration script here
CREATE TABLE IF NOT EXISTS subscriptions(
    id uuid NOT NULL,
    PRIMARY KEY(id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL UNIQUE,
    subscribed_at timestamptz NOT NULL
)