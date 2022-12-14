// @generated automatically by Diesel CLI.

diesel::table! {
    contest_tasks (id) {
        id -> Int4,
        position -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        contest_id -> Int4,
        task_id -> Int4,
    }
}

diesel::table! {
    contests (id) {
        id -> Int4,
        name -> Varchar,
        slug -> Varchar,
        category -> Varchar,
        description -> Varchar,
        start_at -> Timestamp,
        end_at -> Timestamp,
        penalty -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    submits (id) {
        id -> Int4,
        status -> Varchar,
        score -> Nullable<Int4>,
        execution_time -> Nullable<Int4>,
        execution_memory -> Nullable<Int4>,
        compile_message -> Nullable<Varchar>,
        language_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        user_id -> Int4,
        task_id -> Int4,
        contest_id -> Int4,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        name -> Varchar,
        statement -> Varchar,
        part_score -> Nullable<Varchar>,
        constraints -> Varchar,
        input -> Varchar,
        output -> Varchar,
        score -> Int4,
        time_limit -> Int4,
        memory_limit -> Int4,
        task_type -> Varchar,
        is_draft -> Bool,
        is_public -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        author_id -> Int4,
    }
}

diesel::table! {
    testcase_sets (id) {
        id -> Int4,
        name -> Varchar,
        is_sample -> Bool,
        score -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        task_id -> Int4,
    }
}

diesel::table! {
    testcase_testcase_sets (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        testcase_id -> Int4,
        testcase_set_id -> Int4,
    }
}

diesel::table! {
    testcases (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        task_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        encrypted_password -> Varchar,
        display_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(contest_tasks -> contests (contest_id));
diesel::joinable!(contest_tasks -> tasks (task_id));
diesel::joinable!(submits -> contests (contest_id));
diesel::joinable!(submits -> tasks (task_id));
diesel::joinable!(submits -> users (user_id));
diesel::joinable!(tasks -> users (author_id));
diesel::joinable!(testcase_sets -> tasks (task_id));
diesel::joinable!(testcase_testcase_sets -> testcase_sets (testcase_set_id));
diesel::joinable!(testcase_testcase_sets -> testcases (testcase_id));
diesel::joinable!(testcases -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    contest_tasks,
    contests,
    submits,
    tasks,
    testcase_sets,
    testcase_testcase_sets,
    testcases,
    users,
);
