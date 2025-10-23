-- Create table for users
CREATE TABLE users (
    uuid                BLOB NOT NULL UNIQUE,
    email               TEXT NOT NULL UNIQUE,
    password_hash       TEXT NOT NULL,
    first_name          VARCHAR(255) NOT NULL,
    last_name           VARCHAR(255) NOT NULL,
    salt                BLOB NOT NULL,
    argon2_iters        INTEGER NOT NULL,
    argon2_memory_mb    INTEGER NOT NULL,
    argon2_parallelism  INTEGER NOT NULL,
    last_updated        TIMESTAMP,

    PRIMARY KEY (uuid)
);

--
-- Update last_updated for new entries or when old entries are updated
--

CREATE TRIGGER users_trigger_after_insert AFTER INSERT ON users
    BEGIN
        UPDATE users
            SET last_updated = DATETIME('NOW')
            WHERE ROWID = NEW.ROWID;
    END;

CREATE TRIGGER users_trigger_after_update AFTER UPDATE ON users
    BEGIN
        UPDATE users
            SET last_updated = DATETIME('NOW')
            WHERE ROWID = NEW.ROWID;
    END;