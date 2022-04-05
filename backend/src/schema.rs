table! {
    items (name) {
        name -> Text,
        price -> Integer,
        tax -> Float,
    }
}

table! {
    purchases (id) {
        id -> Integer,
        ctime -> Text,
        items -> Text,
        total -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(items, purchases);
