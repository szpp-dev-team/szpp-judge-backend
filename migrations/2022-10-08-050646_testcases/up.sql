CREATE TABLE testcases (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,

    testcase_set_id SERIAL NOT NULL,
    FOREIGN KEY (testcase_set_id)
        REFERENCES testcase_sets (id),
);