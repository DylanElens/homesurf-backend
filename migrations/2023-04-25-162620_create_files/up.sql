-- Your SQL goes here
CREATE TABLE files (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  file_name VARCHAR NOT NULL,
  file_path VARCHAR NOT NULL,
  file_type VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL
)