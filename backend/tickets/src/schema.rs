table! {
    tickets (id) {
        id -> Integer,
        title -> Text,
        body -> Nullable<Text>,
        created -> Integer,
        updated -> Nullable<Integer>,
        creator -> Integer,
        assginee -> Nullable<Integer>,
        stage -> Integer,
        order_index -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    tickets,
    users,
);
