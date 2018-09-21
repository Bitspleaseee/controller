table! {
    categories (id) {
        id -> Integer,
        title -> Varchar,
        description -> Text,
        hidden -> Bool,
    }
}

table! {
    comments (id) {
        id -> Integer,
        thread_id -> Integer,
        parent_id -> Nullable<Integer>,
        user_id -> Integer,
        content -> Text,
        hidden -> Bool,
    }
}

table! {
    threads (id) {
        id -> Integer,
        category_id -> Integer,
        user_id -> Integer,
        title -> Varchar,
        description -> Text,
        timestamp -> Datetime,
        hidden -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        description -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
    }
}

joinable!(comments -> threads (thread_id));
joinable!(comments -> users (user_id));
joinable!(threads -> categories (category_id));
joinable!(threads -> users (user_id));

allow_tables_to_appear_in_same_query!(categories, comments, threads, users,);
