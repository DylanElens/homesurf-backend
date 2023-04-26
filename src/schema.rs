// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Integer,
        file_name -> Text,
        file_path -> Text,
        file_type -> Text,
        created_at -> Timestamp,
    }
}
