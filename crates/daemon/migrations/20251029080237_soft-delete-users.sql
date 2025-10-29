-- Delete old trigger for updated
DROP TRIGGER users_trigger_after_insert;
DROP TRIGGER users_trigger_after_update;

-- Rename updated column
ALTER TABLE users
    RENAME COLUMN last_updated TO created_at;

-- Add a created at column
ALTER TABLE users
    ADD COLUMN updated_at TIMESTAMP;

-- Add a column for soft deleting users
ALTER TABLE users
    ADD COLUMN deleted BOOLEAN DEFAULT FALSE;

-- Add view which only returns non deleted users
CREATE VIEW IF NOT EXISTS users_active AS
    SELECT *
        FROM users
        WHERE users.deleted = FALSE;

-- Add trigger for inserting on view
CREATE TRIGGER users_active_insert INSTEAD OF INSERT ON users_active
    BEGIN
        INSERT INTO users (uuid, username, password_hash, salt, argon2_iters, argon2_memory_mb, argon2_parallelism)
            VALUES (new.uuid, new.username, new.password_hash, new.salt, new.argon2_iters, new.argon2_memory_mb, new.argon2_parallelism);
    END;

-- Add trigger for updating insert and updated
CREATE TRIGGER users_trigger_after_insert AFTER INSERT ON users
    BEGIN
        UPDATE users
            SET
                created_at = DATETIME('NOW'),
                updated_at = DATETIME('NOW')
            WHERE ROWID = NEW.ROWID;
    END;

CREATE TRIGGER users_trigger_after_update AFTER UPDATE ON users
    BEGIN
        UPDATE users
            SET updated_at = DATETIME('NOW')
            WHERE ROWID = NEW.ROWID;
    END;
