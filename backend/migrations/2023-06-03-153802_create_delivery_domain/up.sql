-- Your SQL goes here

CREATE TABLE addresses (
  id SERIAL PRIMARY KEY,
  street VARCHAR NOT NULL,
  city VARCHAR NOT NULL,
  postal_code VARCHAR NOT NULL
);

CREATE TABLE trade_partners (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  price_list VARCHAR NOT NULL
);


-- CREATE TABLE warehouses (
--   id SERIAL PRIMARY KEY,
--   name VARCHAR NOT NULL,
--   trade_partner_id VARCHAR NOT NULL,
--   address_id VARCHAR NOT NULL
-- );
