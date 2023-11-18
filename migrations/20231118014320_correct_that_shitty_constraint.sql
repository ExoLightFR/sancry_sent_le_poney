-- Add migration script here

ALTER TABLE users DROP CONSTRAINT users_pk CASCADE;
ALTER TABLE users DROP CONSTRAINT users_guild_id_key;

ALTER TABLE users ADD PRIMARY KEY (user_id, guild_id);
