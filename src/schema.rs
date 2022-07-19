table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        published_date -> Timestamp,
        last_login -> Timestamp,
    }
}
