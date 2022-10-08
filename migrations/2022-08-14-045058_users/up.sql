CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    encrypted_password VARCHAR NOT NULL,
    display_name VARCHAR,
    session_token VARCHAR,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP
);