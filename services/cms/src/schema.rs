// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Text,
        slug -> Text,
        content -> Text,
        published -> Bool,
        created_at -> Timestamp,
        created_by -> Text,
        last_modified_at -> Timestamp,
        last_modified_by -> Text,
    }
}
