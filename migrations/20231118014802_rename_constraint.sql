-- Add migration script here

ALTER TABLE users RENAME CONSTRAINT users_pkey TO users_pk;
