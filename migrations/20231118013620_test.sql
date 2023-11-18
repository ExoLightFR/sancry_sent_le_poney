-- Add migration script here

CREATE TABLE someTable (
    col1 int NOT NULL,
    col2 int NOT NULL,
    PRIMARY KEY (col1, col2)
);
