table! {
    items (name) {
        name -> Text,
        price -> Int8,
        tax -> Float4,
    }
}

table! {
    purchases (id) {
        id -> Int8,
        ctime -> Timestamp,
        items -> Json,
        total -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(items, purchases,);
