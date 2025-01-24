-- Add migration script here
CREATE TABLE providers (
  id TEXT PRIMARY KEY NOT NULL,
  cookie TEXT
);

insert into providers(id) values ("rutracker");
