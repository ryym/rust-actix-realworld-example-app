table! {
    credentials (id) {
        id -> Int4,
        user_id -> Int4,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        bio -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(credentials -> users (user_id));

allow_tables_to_appear_in_same_query!(
    credentials,
    users,
);
