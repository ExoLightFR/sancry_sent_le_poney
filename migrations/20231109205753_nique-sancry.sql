-- Add migration script here
CREATE TABLE IF NOT EXISTS CHOKBAR (
	id bigserial NOT NULL,
	other_id bigint,
	CONSTRAINT "CHOKBAR_pk" PRIMARY KEY (id)
);

ALTER TABLE CHOKBAR OWNER TO postgres;
