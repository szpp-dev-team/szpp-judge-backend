CREATE TABLE contest_tasks (
    id SERIAL PRIMARY KEY,
    contest_task_id VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,

    contest_id SERIAL NOT NULL,
    task_id SERIAL NOT NULL,
    FOREIGN KEY (contest_id)
        REFERENCES contests (id),
    FOREIGN KEY (task_id)
        REFERENCES tasks (id)
);