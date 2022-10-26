CREATE TABLE contest_tasks (
    id SERIAL PRIMARY KEY NOT NULL,
    position INT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    contest_id INT NOT NULL,
    task_id INT NOT NULL,
    FOREIGN KEY (contest_id)
        REFERENCES contests (id),
    FOREIGN KEY (task_id)
        REFERENCES tasks (id),
    UNIQUE (contest_id, task_id)
);
