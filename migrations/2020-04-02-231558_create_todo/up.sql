-- Your SQL goes here
CREATE TABLE todo (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  completed BOOLEAN DEFAULT 0 NOT NULL,
  created_at DATETIME NOT NULL
);
