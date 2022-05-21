table! {
    Microbits (id) {
        id -> Text,
        user_id -> Nullable<Int4>,
        active_begin_hours -> Int4,
        active_begin_minutes -> Int4,
        active_end_hours -> Int4,
        active_end_minutes -> Int4,
    }
}

table! {
    Users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
    }
}

joinable!(Microbits -> Users (user_id));

allow_tables_to_appear_in_same_query!(
    Microbits,
    Users,
);

