DELETE
FROM contests
WHERE id = 1;
INSERT INTO contests (name, slug, category, description, start_at, end_at, penalty, created_at)
VALUES ('Development Test Contest',
        'devtc',
        'dev',
        '開発テストコンテスト',
        '2022-10-01 00:00:00',
        '2024-10-01 00:00:00',
        300,
        now());
SELECT setval('contests_id_seq', nextval('contests_id_seq'), false);

DELETE
FROM users
WHERE id = 1;
INSERT INTO users (username, encrypted_password, display_name, created_at)
VALUES ('test_user',
        '5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8',
        NULL,
        now())
ON CONFLICT (username)
    DO NOTHING;
SELECT setval('users_id_seq', nextval('users_id_seq'), false);

DELETE
FROM tasks
WHERE id = 1;
INSERT INTO tasks (name, statement, constraints, part_score, input, output, score, time_limit, memory_limit,
                   task_type,
                   is_draft, is_public, created_at, author_id)
VALUES ('Simple Adding',
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
        1);
SELECT setval('tasks_id_seq', nextval('tasks_id_seq'), false);

DELETE
FROM testcases
WHERE id = 1
   OR id = 2
   OR id = 3;
INSERT INTO testcases (name, created_at, task_id)
VALUES ('sample1.txt',
        now(),
        1);
INSERT INTO testcases (name, created_at, task_id)
VALUES ('sample2.txt',
        now(),
        1);
INSERT INTO testcases (name, created_at, task_id)
VALUES ('sample3.txt',
        now(),
        1);
SELECT setval('testcases_id_seq', nextval('testcases_id_seq'), false);

DELETE
FROM testcase_sets
WHERE id = 1
   OR id = 2;
INSERT INTO testcase_sets (name, is_sample, score, created_at, task_id)
VALUES ('sample', true, 0, now(), 1);
INSERT INTO testcase_sets (name, is_sample, score, created_at, task_id)
VALUES ('all', true, 0, now(), 1);
SELECT setval('testcase_sets_id_seq', nextval('testcase_sets_id_seq'), false);

DELETE
FROM testcase_testcase_sets
WHERE id = 1
   OR id = 2
   OR id = 3
   OR id = 4
   OR id = 5
   OR id = 6;
INSERT INTO testcase_testcase_sets (created_at, testcase_id, testcase_set_id)
VALUES (now(), 1, 1);
INSERT INTO testcase_testcase_sets (created_at, testcase_id, testcase_set_id)
VALUES (now(), 2, 1);
INSERT INTO testcase_testcase_sets (created_at, testcase_id, testcase_set_id)
VALUES (now(), 3, 1);
INSERT INTO testcase_testcase_sets (created_at, testcase_id, testcase_set_id)
VALUES (now(), 1, 2);
INSERT INTO testcase_testcase_sets (created_at, testcase_id, testcase_set_id)
VALUES (now(), 2, 2);
INSERT INTO testcase_testcase_sets (created_at, testcase_id, testcase_set_id)
VALUES (now(), 3, 2);
SELECT setval('testcase_testcase_sets_id_seq', nextval('testcase_testcase_sets_id_seq'), false);

DELETE
FROM submits
WHERE id = 1
   OR id = 2
   OR id = 3
   OR id = 4
   OR id = 5;
INSERT INTO submits (created_at, user_id, task_id, contest_id, status, language_id)
VALUES (now(), 1, 1, 1, 'WJ', 'cpp');
INSERT INTO submits (created_at, user_id, task_id, contest_id, status, language_id)
VALUES (now(), 1, 1, 1, 'WJ', 'cpp');
INSERT INTO submits (created_at, user_id, task_id, contest_id, status, language_id)
VALUES (now(), 1, 1, 1, 'WJ', 'cpp');
INSERT INTO submits (created_at, user_id, task_id, contest_id, status, language_id)
VALUES (now(), 1, 1, 1, 'WJ', 'cpp');
INSERT INTO submits (created_at, user_id, task_id, contest_id, status, language_id)
VALUES (now(), 1, 1, 1, 'WJ', 'cpp');
SELECT setval('submits_id_seq', nextval('submits_id_seq'), false);