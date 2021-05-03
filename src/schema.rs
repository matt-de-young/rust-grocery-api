table! {
    items (id) {
        id -> Int4,
        body -> Varchar,
        description -> Nullable<Varchar>,
        quantity -> Int4,
        complete -> Bool,
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

allow_tables_to_appear_in_same_query!(
    items,
    posts,
);
