table! {
    posts (id) {
        id -> Integer,
        body -> Text,
        #[sql_name = "title"]
        my_title -> Varchar,
        published -> Bool,
        create_time -> Nullable<Datetime>,
        create_time_ts -> Timestamp,
        spent -> Decimal,
        spent_num -> Decimal,
        array_data -> Nullable<Binary>,
    }
}