CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    statement VARCHAR NOT NULL,
    part_score VARCHAR,
    constraints VARCHAR NOT NULL,
    input VARCHAR NOT NULL,
    output VARCHAR NOT NULL,
    score INT NOT NULL,
    time_limit INT NOT NULL,
    memory_limit INT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    contest_id SERIAL,
    author_id SERIAL,
    FOREIGN KEY (contest_id)
        REFERENCES contests (id),
    FOREIGN KEY (author_id)
        REFERENCES users (id)
);