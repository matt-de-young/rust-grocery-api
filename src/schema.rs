table! {
    items (id) {
        id -> Int4,
        body -> Varchar,
        description -> Nullable<Varchar>,
        quantity -> Int4,
        complete -> Bool,
        user_id -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Nullable<Varchar>,
    }
}

joinable!(items -> users (id));

allow_tables_to_appear_in_same_query!(
    items,
    posts,
    users,
);
