CREATE TABLE testcase_testcase_sets (
    id SERIAL PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    testcase_id SERIAL NOT NULL,
    testcase_set_id SERIAL NOT NULL,
    FOREIGN KEY (testcase_id)
        REFERENCES testcases (id),
    FOREIGN KEY (testcase_set_id)
        REFERENCES testcase_sets (id)
);