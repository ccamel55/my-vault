-- Add migration script here
CREATE TABLE my_table (
    name        TEXT NOT NULL,
    password    TEXT NOT NULL,

    PRIMARY KEY (name)
);

INSERT INTO my_table (name, password)
    VALUES
        ('bob',     '123'),
        ('alice',   '456'),
        ('steve',   '789'),
        ('fart',    '101112');