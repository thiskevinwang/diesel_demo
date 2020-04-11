table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        user_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        name -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
