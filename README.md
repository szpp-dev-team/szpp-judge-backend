# szpp-judge-backend

## setup

### 1. migration

```shell
$ diesel --version
diesel 2.0.1

$ diesel migration list
Migrations:
  [ ] 00000000000000_diesel_initial_setup
  [ ] 2022-08-14-045058_users
  [ ] 2022-10-08-020600_contests
  [ ] 2022-10-08-020606_tasks
  [ ] 2022-10-08-021944_submits
  [ ] 2022-10-08-030615_contest_tasks
  [ ] 2022-10-08-050640_testcase_sets
  [ ] 2022-10-08-050646_testcases

$ diesel run

$ diesel migration list
Migrations:
  [X] 00000000000000_diesel_initial_setup
  [X] 2022-08-14-045058_users
  [X] 2022-10-08-020600_contests
  [X] 2022-10-08-020606_tasks
  [X] 2022-10-08-021944_submits
  [X] 2022-10-08-030615_contest_tasks
  [X] 2022-10-08-050640_testcase_sets
  [X] 2022-10-08-050646_testcases
```

### 2. launch postgresql

```shell
$ docker compose up -d db
```

### 3. launch server

```shell
$ cargo run
```
