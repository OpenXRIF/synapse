-- Your SQL goes here
CREATE TABLE prompt_formats (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  format_name VARCHAR NOT NULL,
  prompt TEXT NOT NULL
)
