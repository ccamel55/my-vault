CREATE TABLE IF NOT EXISTS users (
    uuid    BLOB PRIMARY KEY NOT NULL,
    name    VARCHAR(255) NOT NULL UNIQUE,

    -- Bitwarden login information
    email           VARCHAR(255) NOT NULL UNIQUE,
    password_hash   BLOB NOT NULL
);