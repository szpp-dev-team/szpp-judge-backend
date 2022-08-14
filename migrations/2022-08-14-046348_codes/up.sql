CREATE TABLE codes (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    code_url VARCHAR NOT NULL,
    language VARCHAR,
    author_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    FOREIGN KEY (author_id) references users(id)
)