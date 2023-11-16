-- Add migration script here

ALTER TABLE users
ADD COLUMN forced_name VARCHAR(32);
