table! {
    categories (id) {
        id -> Unsigned<Integer>,
        title -> Varchar,
        description -> Text,
        hidden -> Bool,
    }
}

table! {
    comments (id) {
        id -> Unsigned<Integer>,
        thread_id -> Unsigned<Integer>,
        parent_id -> Nullable<Unsigned<Integer>>,
        user_id -> Unsigned<Integer>,
        content -> Text,
        hidden -> Bool,
    }
}

table! {
    threads (id) {
        id -> Unsigned<Integer>,
        category_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        title -> Varchar,
        description -> Text,
        timestamp -> Datetime,
        hidden -> Bool,
    }
}

table! {
    users (id) {
        id -> Unsigned<Integer>,
        username -> Varchar,
        description -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
    }
}

joinable!(comments -> threads (thread_id));
joinable!(comments -> users (user_id));
joinable!(threads -> categories (category_id));
joinable!(threads -> users (user_id));

allow_tables_to_appear_in_same_query!(
    categories,
    comments,
    threads,
    users,
);
