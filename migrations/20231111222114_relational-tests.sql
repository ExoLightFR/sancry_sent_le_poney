-- Add migration script here
CREATE TABLE FOOBAR (
	id BIGSERIAL NOT NULL PRIMARY KEY,
	trucmuche TEXT,
	chokbar bigserial,
	CONSTRAINT fk_chokbar FOREIGN KEY(chokbar) REFERENCES CHOKBAR(id)
);
