-- Your SQL goes here

CREATE TYPE ParcelSize AS ENUM ('S', 'M', 'L');

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

CREATE TABLE warehouses (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  trade_partner_id SERIAL NOT NULL REFERENCES trade_partners(id),
  address_id SERIAL NOT NULL REFERENCES addresses(id)
);

CREATE TABLE parcels (
  id SERIAL PRIMARY KEY,
  recipient_name VARCHAR NOT NULL,
  recipient_email VARCHAR NOT NULL,
  recipient_phone VARCHAR NOT NULL,
  recipient_address_id SERIAL NOT NULL REFERENCES addresses(id),
  warehouse_id SERIAL NOT NULL REFERENCES warehouses(id),
  pickup_date DATE NOT NULL,
  size ParcelSize NOT NULL
);

CREATE TABLE status_records (
  id SERIAL PRIMARY KEY,
  parcel_id SERIAL NOT NULL REFERENCES parcels(id),
  status VARCHAR NOT NULL,
  creation_time TIMESTAMP NOT NULL
);
