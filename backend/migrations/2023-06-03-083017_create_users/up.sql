-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  firstname VARCHAR NOT NULL,
  lastname VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  phone VARCHAR NOT NULL,
  pass_hash VARCHAR NOT NULL
);

CREATE TABLE roles (
  role_name VARCHAR NOT NULL,
  user_id SERIAL NOT NULL REFERENCES users(id),
  PRIMARY KEY(role_name, user_id)
);

INSERT INTO users (id, firstname, lastname, email, phone, pass_hash)
VALUES (1, 'Jan', 'Testowy', 'jan@testowy.com', '123456789','$argon2id$v=19$m=19456,t=2,p=1$BUPdNw7uZdFKdR4tAkvKvg$cD87MuP3GhET6ofAFqZp1TDTd9rBplFI5p2M8r+ssog');

INSERT INTO roles (role_name, user_id)
VALUES ('"Admin"', 1);