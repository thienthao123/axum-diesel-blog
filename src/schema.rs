// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        post_id -> Int4,
        name -> Varchar,
        slug -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        hashed_password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(posts -> users (user_id));
diesel::joinable!(tags -> posts (post_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    tags,
    users,
);
