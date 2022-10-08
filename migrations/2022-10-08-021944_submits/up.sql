CREATE TABLE submits (
    id SERIAL PRIMARY KEY,
    status VARCHAR NOT NULL,
    source_id VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,

    user_id VARCHAR NOT NULL,
    language_id SERIAL NOT NULL,
    task_id SERIAL NOT NULL,
    FOREIGN KEY (user_id)
        REFERENCES users (id),
    FOREIGN KEY (language_id)
        REFERENCES languages (id),
    FOREIGN KEY (task_id)
        REFERENCES tasks (id)
);