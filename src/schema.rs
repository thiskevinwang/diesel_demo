table! {
    Attempts (id) {
        id -> Int4,
        created -> Timestamp,
        updated -> Nullable<Timestamp>,
        deleted -> Nullable<Timestamp>,
        grade -> Int4,
        send -> Nullable<Bool>,
        flash -> Nullable<Bool>,
        date -> Nullable<Timestamp>,
        userId -> Int4,
    }
}

table! {
    Comments (id) {
        id -> Int4,
        created -> Timestamp,
        updated -> Nullable<Timestamp>,
        deleted -> Nullable<Timestamp>,
        #[sql_name = "type"]
        type_ -> Varchar,
        body -> Varchar,
        url -> Varchar,
        userId -> Int4,
    }
}

table! {
    Reactions (id) {
        id -> Int4,
        created -> Timestamp,
        updated -> Nullable<Timestamp>,
        deleted -> Nullable<Timestamp>,
        #[sql_name = "type"]
        type_ -> Varchar,
        variant -> Reactions_variant_enum,
        commentId -> Int4,
        userId -> Int4,
    }
}

table! {
    Users (id) {
        id -> Int4,
        created -> Timestamp,
        updated -> Nullable<Timestamp>,
        deleted -> Nullable<Timestamp>,
        #[sql_name = "type"]
        type_ -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        avatar_url -> Nullable<Varchar>,
        last_password_request -> Nullable<Timestamp>,
        verified_date -> Nullable<Timestamp>,
        banned -> Nullable<Bool>,
    }
}

joinable!(Attempts -> Users (userId));
joinable!(Comments -> Users (userId));
joinable!(Reactions -> Comments (commentId));
joinable!(Reactions -> Users (userId));

allow_tables_to_appear_in_same_query!(
    Attempts,
    Comments,
    Reactions,
    Users,
);
