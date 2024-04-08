-- Add migration script here
CREATE TABLE subscriber
(
    id         SERIAL PRIMARY KEY NOT NULL,
    name       VARCHAR(255)       NOT NULL,
    email      VARCHAR(255)       NOT NULL,
    expire_at  TIMESTAMP          NULL,
    created_at TIMESTAMP          NOT NULL DEFAULT CURRENT_TIMESTAMP
);
