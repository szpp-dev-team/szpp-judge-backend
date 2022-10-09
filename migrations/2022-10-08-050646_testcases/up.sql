CREATE TABLE testcases (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,

    task_id SERIAL NOT NULL,
    FOREIGN KEY (task_id)
        REFERENCES tasks (id)
);