table! {
    codes (id) {
        id -> Int4,
        title -> Varchar,
        code_url -> Varchar,
        language -> Nullable<Varchar>,
        author_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(codes -> users (author_id));

allow_tables_to_appear_in_same_query!(
    codes,
    users,
);
