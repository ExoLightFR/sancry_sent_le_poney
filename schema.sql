-- Database generated with pgModeler (PostgreSQL Database Modeler).
-- pgModeler version: 1.0.6
-- PostgreSQL version: 16.0
-- Project Site: pgmodeler.io
-- Model Author: ---
-- Tablespaces creation must be performed outside a multi lined SQL file. 
-- These commands were put in this file only as a convenience.
-- 
-- object: niquesancry | type: TABLESPACE --
-- DROP TABLESPACE IF EXISTS niquesancry CASCADE;

-- CREATE TABLESPACE niquesancry
-- 	OWNER postgres
-- 	LOCATION 'niquesancry';

-- ddl-end --



-- Database creation must be performed outside a multi lined SQL file. 
-- These commands were put in this file only as a convenience.
-- 
-- object: new_database | type: DATABASE --
-- DROP DATABASE IF EXISTS new_database;
-- CREATE DATABASE new_database;
-- ddl-end --


-- object: "GuildData" | type: TABLE --
DROP TABLE IF EXISTS "GuildData" CASCADE;
CREATE TABLE "GuildData" (
	id bigserial NOT NULL,
	target_id bigint,
	fart_target bigint,
	CONSTRAINT "GuildData_pk" PRIMARY KEY (id)
);
-- ddl-end --
COMMENT ON COLUMN "GuildData".id IS E'Guild ID';
-- ddl-end --
ALTER TABLE "GuildData" OWNER TO postgres;
-- ddl-end --

-- object: "Mutes" | type: TABLE --
DROP TABLE IF EXISTS "Mutes" CASCADE;
CREATE TABLE "Mutes" (
	user_id bigserial NOT NULL,
	guild_id bigserial NOT NULL,
	from_id bigserial NOT NULL,
	reason varchar,
	"id_GuildData" bigint

);
-- ddl-end --
ALTER TABLE "Mutes" OWNER TO postgres;
-- ddl-end --

-- object: "GuildData_fk" | type: CONSTRAINT --
ALTER TABLE "Mutes" DROP CONSTRAINT IF EXISTS "GuildData_fk" CASCADE;
ALTER TABLE "Mutes" ADD CONSTRAINT "GuildData_fk" FOREIGN KEY ("id_GuildData")
REFERENCES "GuildData" (id) MATCH FULL
ON DELETE SET NULL ON UPDATE CASCADE;
-- ddl-end --

-- object: "Users" | type: TABLE --
DROP TABLE IF EXISTS "Users" CASCADE;
CREATE TABLE "Users" (
	id bigserial NOT NULL,
	mute_until date,
	points integer,
	join_sound varchar,
	"id_GuildData" bigint,
	CONSTRAINT "Users_pk" PRIMARY KEY (id)
);
-- ddl-end --
COMMENT ON COLUMN "Users".join_sound IS E'some stupid shit or something';
-- ddl-end --
ALTER TABLE "Users" OWNER TO postgres;
-- ddl-end --

-- object: "GuildData_fk" | type: CONSTRAINT --
ALTER TABLE "Users" DROP CONSTRAINT IF EXISTS "GuildData_fk" CASCADE;
ALTER TABLE "Users" ADD CONSTRAINT "GuildData_fk" FOREIGN KEY ("id_GuildData")
REFERENCES "GuildData" (id) MATCH FULL
ON DELETE SET NULL ON UPDATE CASCADE;
-- ddl-end --

-- object: "Users_uq" | type: CONSTRAINT --
ALTER TABLE "Users" DROP CONSTRAINT IF EXISTS "Users_uq" CASCADE;
ALTER TABLE "Users" ADD CONSTRAINT "Users_uq" UNIQUE ("id_GuildData");
-- ddl-end --

-- object: pins | type: TABLE --
DROP TABLE IF EXISTS pins CASCADE;
CREATE TABLE pins (
	id bigserial NOT NULL,
	guild_id bigserial NOT NULL,
	from_id bigserial NOT NULL,
	reactions smallint,
	"id_GuildData" bigint

);
-- ddl-end --
COMMENT ON TABLE pins IS E'List of starred/pinned messages by users';
-- ddl-end --
COMMENT ON COLUMN pins.reactions IS E'Number of reactions that request a pin';
-- ddl-end --
ALTER TABLE pins OWNER TO postgres;
-- ddl-end --

-- object: "GuildData_fk" | type: CONSTRAINT --
ALTER TABLE pins DROP CONSTRAINT IF EXISTS "GuildData_fk" CASCADE;
ALTER TABLE pins ADD CONSTRAINT "GuildData_fk" FOREIGN KEY ("id_GuildData")
REFERENCES "GuildData" (id) MATCH FULL
ON DELETE SET NULL ON UPDATE CASCADE;
-- ddl-end --
