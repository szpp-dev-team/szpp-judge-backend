CREATE TABLE constests (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    slug VARCHAR NOT NULL,
    category VARCHAR NOT NULL,
    constraints VARCHAR NOT NULL,
    start_at TIMESTAMP NOT NULL,
    end_at TIMESTAMP NOT NULL,
    penalty int NOT NULL,
    
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
);