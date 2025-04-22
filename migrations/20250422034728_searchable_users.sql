-- Add migration script here

ALTER TABLE users
DROP COLUMN username;

CREATE UNIQUE INDEX unique_email_lowercase ON users (LOWER(email));
