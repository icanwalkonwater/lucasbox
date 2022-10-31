// @generated automatically by Diesel CLI.

diesel::table! {
    collection_items (id) {
        id -> Int4,
        collection_id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        created_by -> Nullable<Uuid>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    collections (id) {
        id -> Int4,
        parent_id -> Nullable<Int4>,
        inline -> Bool,
        level -> Int2,
        name -> Text,
        description -> Nullable<Text>,
        created_by -> Nullable<Uuid>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    item_files (id) {
        id -> Int4,
        collection_item_id -> Int4,
        name -> Text,
        filepath -> Text,
        created_by -> Nullable<Uuid>,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    tag_collection_items (tag_id, collection_item_id) {
        tag_id -> Int4,
        collection_item_id -> Int4,
    }
}

diesel::table! {
    tag_collections (tag_id, collection_id) {
        tag_id -> Int4,
        collection_id -> Int4,
    }
}

diesel::table! {
    tag_item_files (tag_id, item_file_id) {
        tag_id -> Int4,
        item_file_id -> Int4,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        label -> Text,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

diesel::table! {
    user_refresh_tokens (user_id, token) {
        user_id -> Uuid,
        token -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        admin -> Bool,
        username -> Varchar,
        password -> Text,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::joinable!(collection_items -> collections (collection_id));
diesel::joinable!(collection_items -> users (created_by));
diesel::joinable!(collections -> users (created_by));
diesel::joinable!(item_files -> collection_items (collection_item_id));
diesel::joinable!(item_files -> users (created_by));
diesel::joinable!(tag_collection_items -> collection_items (collection_item_id));
diesel::joinable!(tag_collection_items -> tags (tag_id));
diesel::joinable!(tag_collections -> collections (collection_id));
diesel::joinable!(tag_collections -> tags (tag_id));
diesel::joinable!(tag_item_files -> item_files (item_file_id));
diesel::joinable!(tag_item_files -> tags (tag_id));
diesel::joinable!(user_refresh_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    collection_items,
    collections,
    item_files,
    tag_collection_items,
    tag_collections,
    tag_item_files,
    tags,
    user_refresh_tokens,
    users,
);
