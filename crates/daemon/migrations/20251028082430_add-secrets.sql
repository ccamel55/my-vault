-- Add table for secrets collection
CREATE TABLE collections (
    uuid    BLOB NOT NULL UNIQUE,
    name    TEXT NOT NULL UNIQUE,

    PRIMARY KEY (uuid)
);

-- Add table for secrets
CREATE TABLE secrets (
    uuid        BLOB NOT NULL UNIQUE,
    name        TEXT NOT NULL UNIQUE,
    key         TEXT,
    description TEXT,
    secret      TEXT NOT NULL,
    secret_type INTEGER NOT NULL,

    PRIMARY KEY (uuid)
);

-- Add table for secret source
CREATE TABLE sources (
    uuid        BLOB NOT NULL UNIQUE,
    name        TEXT NOT NULL UNIQUE,
    description TEXT,
    source_type INTEGER NOT NULL,

    -- Authentication data for source
    source_auth         TEXT,
    source_auth_type    INTEGER NOT NULL,

    -- Timestamp marking changes
    created_at TIMESTAMP,
    updated_at TIMESTAMP,

    PRIMARY KEY (uuid)
);

-- Add table for correlating secret with a source
CREATE TABLE source_secrets (
    uuid_source BLOB NOT NULL,
    uuid_secret BLOB NOT NULL,

    FOREIGN KEY (uuid_source) REFERENCES sources(uuid),
    FOREIGN KEY (uuid_secret) REFERENCES secrets(uuid)
);

-- Add table for correlating source with a collection
CREATE TABLE collection_source (
    uuid_collection BLOB NOT NULL,
    uuid_source     BLOB NOT NULL,

    FOREIGN KEY (uuid_collection) REFERENCES collections(uuid),
    FOREIGN KEY (uuid_source) REFERENCES sources(uuid)
);

--
-- Add trigger for updating insert and updated
--

CREATE TRIGGER sources_trigger_after_insert AFTER INSERT ON sources
    BEGIN
        UPDATE sources
            SET
                created_at = DATETIME('NOW'),
                updated_at = DATETIME('NOW')
            WHERE ROWID = NEW.ROWID;
    END;

CREATE TRIGGER sources_trigger_after_update AFTER UPDATE ON sources
    BEGIN
        UPDATE sources
            SET updated_at = DATETIME('NOW')
            WHERE ROWID = NEW.ROWID;
    END;
