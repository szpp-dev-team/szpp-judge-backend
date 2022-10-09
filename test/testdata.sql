INSERT INTO contests (name, slug, category, description, start_at, end_at, penalty, created_at)
VALUES ('Development Test Contest',
        'devtc',
        'dev',
        '開発テストコンテスト',
        '2022-10-01 00:00:00',
        '2024-10-01 00:00:00',
        300,
        now())
ON CONFLICT (slug)
    DO NOTHING;

INSERT INTO users (username, encrypted_password, display_name, session_token, created_at)
VALUES ('test_user',
        '5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8',
        NULL,
        NULL,
        now())
ON CONFLICT (username)
    DO NOTHING;

INSERT INTO tasks (name, statement, constraints, part_score, input, output, score, time_limit, memory_limit, created_at)
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
        now());
