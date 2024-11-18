// @generated automatically by Diesel CLI.

diesel::table! {
    people (id) {
        id -> Int4,
        name -> Varchar,
        born_at -> Nullable<Date>,
        updated_at -> Timestamp,
    }
}
