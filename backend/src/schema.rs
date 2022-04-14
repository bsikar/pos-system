table! {
    items (name) {
        name -> Text,
        price -> Integer,
        tax -> Float,
        #[sql_name = "type"]
        type_ -> Text,
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

allow_tables_to_appear_in_same_query!(items, purchases,);
