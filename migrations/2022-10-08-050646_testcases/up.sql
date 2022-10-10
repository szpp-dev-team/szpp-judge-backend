CREATE TABLE testcases (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,

    task_id SERIAL NOT NULL,
    FOREIGN KEY (task_id)
        REFERENCES tasks (id)
);