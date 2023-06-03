-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  firstname VARCHAR NOT NULL,
  lastname VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  phone VARCHAR NOT NULL,
  pass_hash VARCHAR NOT NULL
)

