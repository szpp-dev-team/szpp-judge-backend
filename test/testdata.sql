DELETE
FROM contests
WHERE id = 1;
INSERT INTO contests (id, name, slug, category, description, start_at, end_at, penalty, created_at)
VALUES (1,
        'Development Test Contest',
        'devtc',
        'dev',
        '開発テストコンテスト',
        '2022-10-01 00:00:00',
        '2024-10-01 00:00:00',
        300,
        now());

DELETE
FROM users
WHERE id = 1;
INSERT INTO users (id, username, encrypted_password, display_name, session_token, created_at)
VALUES (1, 'test_user',
        '5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8',
        NULL,
        NULL,
        now())
ON CONFLICT (username)
    DO NOTHING;

DELETE
FROM tasks
WHERE id = 1;
INSERT INTO tasks (id, name, statement, constraints, part_score, input, output, score, time_limit, memory_limit,
                   task_type,
                   is_draft, is_public, created_at, contest_id, author_id)
VALUES (1, 'Simple Adding',
        E'整数 $A$, $B$ が与えられます。\n$A + B$ を求めてください。',
        E'- $0 \leq A, \ B \leq 10^{10^5}$',
        E'- $0 \leq A, B \leq 2^{63}-1$ のケースに正解した場合は、$100$ 点が与えられる。\n- $0 \leq A, B \leq 10^{10^5}$ のケースに正解した場合は、上とは別に $200$ 点が与えられる。',
        E'入力は以下の形式で標準入力から与えられる。

```
A B
```',
        E'答えを出力せよ。',
        300,
        2000,
        256,
        'contest',
        false,
        true,
        now(),
        1,
        1);

DELETE
FROM testcases
WHERE id = 1
   OR id = 2
   OR id = 3;
INSERT INTO testcases (id, name, created_at, task_id)
VALUES (1, 'sample1.txt',
        now(),
        1);
INSERT INTO testcases (id, name, created_at, task_id)
VALUES (2, 'sample2.txt',
        now(),
        1);
INSERT INTO testcases (id, name, created_at, task_id)
VALUES (3, 'sample3.txt',
        now(),
        1);

DELETE
FROM testcase_sets
WHERE id = 1;
INSERT INTO testcase_sets (id, name, is_sample, score, created_at, task_id)
VALUES (1, 'sample', true, 0, now(), 1);

DELETE
FROM testcase_testcase_sets
WHERE id = 1
   OR id = 2
   OR id = 3;
INSERT INTO testcase_testcase_sets (id, created_at, testcase_id, testcase_set_id)
VALUES (1, now(), 1, 1);
INSERT INTO testcase_testcase_sets (id, created_at, testcase_id, testcase_set_id)
VALUES (2, now(), 2, 1);
INSERT INTO testcase_testcase_sets (id, created_at, testcase_id, testcase_set_id)
VALUES (3, now(), 3, 1);
