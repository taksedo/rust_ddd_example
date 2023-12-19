-- Your SQL goes here

CREATE TABLE IF NOT EXISTS shop.meal (
    id BIGINT PRIMARY KEY NOT NULL,
    name VarChar UNIQUE NOT NULL,
    description VarChar,
    removed BOOLEAN NOT NULL,
    price NUMERIC NOT NULL,
    version BIGINT NOT NULL
)