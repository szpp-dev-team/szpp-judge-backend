CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL UNIQUE,
    encrypted_password VARCHAR NOT NULL,
    display_name VARCHAR,
    session_token VARCHAR UNIQUE,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP
);