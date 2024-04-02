// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Text,
        slug -> Text,
        content -> Text,
        published -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        created_by -> Text,
        last_modified_at -> Nullable<Timestamp>,
        last_modified_by -> Text,
    }
}
