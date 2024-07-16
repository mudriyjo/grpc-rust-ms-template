-- Add migration script here
CREATE TABLE IF NOT EXISTS user_information (
    id SERIAL PRIMARY KEY,
    user_name varchar(255) NOT NULL,
    user_second_name varchar(255) NOT NULL,
    phone varchar(255) NOT NULL,
    user_address varchar(512) NOT NULL
);