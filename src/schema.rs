table! {
    Microbits (MicrobitID) {
        MicrobitID -> Int4,
        F_AccountID -> Nullable<Int4>,
    }
}

table! {
    Users (AccountID) {
        AccountID -> Int4,
        Email -> Text,
        Password -> Text,
    }
}

joinable!(Microbits -> Users (F_AccountID));

allow_tables_to_appear_in_same_query!(
    Microbits,
    Users,
);
