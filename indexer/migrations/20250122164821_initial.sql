-- Add migration script here
CREATE TABLE providers (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT UNIQUE
);

CREATE TABLE cookies (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  provider_id INTEGER NOT NULL,
  cookie TEXT,
  FOREIGN KEY(provider_id) REFERENCES providers(id),
);
