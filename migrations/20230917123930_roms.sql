CREATE TABLE IF NOT EXISTS roms (
    id integer PRIMARY KEY NOT NULL,
    name varchar NOT NULL,
    sha1 varchar(40) NOT NULL
);
