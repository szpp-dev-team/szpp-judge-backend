CREATE TABLE submits (
    id SERIAL PRIMARY KEY,
    status VARCHAR NOT NULL,
    score INT,
    execution_time INT,
    execution_memory INT,
    compile_message VARCHAR,
    language_id VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,

    user_id SERIAL NOT NULL,
    task_id SERIAL NOT NULL,
    contest_id SERIAL NOT NULL,
    FOREIGN KEY (user_id)
        REFERENCES users (id),
    FOREIGN KEY (task_id)
        REFERENCES tasks (id),
    FOREIGN KEY (contest_id)
        REFERENCES contests (id)
);