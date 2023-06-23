-- Your SQL goes here
CREATE TABLE users
(
    id         UUID         NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    name       VARCHAR(255) NOT NULL,
    email      VARCHAR(255) NOT NULL UNIQUE,
    password   VARCHAR(255) NOT NULL,
    created_at TIMESTAMP    NOT NULL             DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP    NOT NULL             DEFAULT CURRENT_TIMESTAMP
)