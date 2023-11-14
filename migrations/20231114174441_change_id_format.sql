-- Add migration script here

DROP TABLE guilds CASCADE;
DROP TABLE users CASCADE;
DROP TABLE pins CASCADE;

CREATE TABLE public.guilds (
	guild_id numeric(64) NOT NULL,
	sing_id numeric(64),
	fart_id numeric(64),
	CONSTRAINT guilds_pk PRIMARY KEY (guild_id)
);
-- ddl-end --
ALTER TABLE public.guilds OWNER TO postgres;
-- ddl-end --

-- object: public.pins | type: TABLE --
-- DROP TABLE IF EXISTS public.pins CASCADE;
CREATE TABLE public.pins (
	msg_id numeric(64) NOT NULL,
	num_reactions integer NOT NULL DEFAULT 0,
	last_react_activity date NOT NULL DEFAULT CURRENT_DATE,
	guild_id numeric(64),
	user_id numeric(64),
	CONSTRAINT pins_pk PRIMARY KEY (msg_id)
);
-- ddl-end --
ALTER TABLE public.pins OWNER TO postgres;
-- ddl-end --

-- object: guilds_fk | type: CONSTRAINT --
-- ALTER TABLE public.pins DROP CONSTRAINT IF EXISTS guilds_fk CASCADE;
ALTER TABLE public.pins ADD CONSTRAINT guilds_fk FOREIGN KEY (guild_id)
REFERENCES public.guilds (guild_id) MATCH FULL
ON DELETE SET NULL ON UPDATE CASCADE;
-- ddl-end --

-- object: public.users | type: TABLE --
-- DROP TABLE IF EXISTS public.users CASCADE;
CREATE TABLE public.users (
	user_id numeric(64) NOT NULL,
	points integer NOT NULL DEFAULT 0,
	join_sound text,
	CONSTRAINT users_pk PRIMARY KEY (user_id)
);
-- ddl-end --
ALTER TABLE public.users OWNER TO postgres;
-- ddl-end --

-- object: users_fk | type: CONSTRAINT --
-- ALTER TABLE public.pins DROP CONSTRAINT IF EXISTS users_fk CASCADE;
ALTER TABLE public.pins ADD CONSTRAINT users_fk FOREIGN KEY (user_id)
REFERENCES public.users (user_id) MATCH FULL
ON DELETE SET NULL ON UPDATE CASCADE;
-- ddl-end --
