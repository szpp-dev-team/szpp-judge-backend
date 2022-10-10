CREATE TABLE testcase_sets (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    is_sample BOOLEAN NOT NULL,
    score INT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    task_id SERIAL NOT NULL,
    FOREIGN KEY (task_id)
        REFERENCES tasks (id)
);