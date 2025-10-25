-- Drop columns which contain human specific details from user table
ALTER TABLE users
    RENAME COLUMN email to username;

ALTER TABLE users
    DROP COLUMN first_name;

ALTER TABLE users
    DROP COLUMN last_name;