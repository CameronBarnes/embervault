// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "file_availability"))]
    pub struct FileAvailability;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "path_format"))]
    pub struct PathFormat;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "storage_type"))]
    pub struct StorageType;
}

diesel::table! {
    content (id) {
        id -> Int4,
        dhash -> Int8,
        phash -> Int8,
        title -> Text,
        description -> Text,
        favorite -> Bool,
        private -> Bool,
        restricted_tag_count -> Int4,
        restricted -> Bool,
        duration_viewed_ms -> Int8,
        views -> Int4,
        points -> Int4,
        score -> Int4,
        has_manually_added_tags -> Bool,
        media_type -> Text,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
        duration_ms -> Nullable<Int4>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    content_file_hash (id) {
        id -> Int4,
        content_id -> Int4,
        hash_type -> Text,
        hash_value -> Bytea,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FileAvailability;

    content_files (content_id) {
        content_id -> Int4,
        storage_root_id -> Int4,
        relative_path -> Bytea,
        file_size -> Int8,
        modified_at -> Timestamptz,
        availability -> FileAvailability,
    }
}

diesel::table! {
    content_frame_hash (id) {
        id -> Int4,
        content_id -> Int4,
        frame_index -> Int4,
        dhash -> Int8,
        phash -> Int8,
    }
}

diesel::table! {
    content_tag (content_id, tag_id) {
        content_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StorageType;
    use super::sql_types::PathFormat;

    storage_roots (id) {
        id -> Int4,
        kind -> StorageType,
        path_format -> PathFormat,
        path -> Bytea,
    }
}

diesel::table! {
    tag_aliases (id) {
        id -> Int4,
        name_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    tag_categories (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        permanent -> Bool,
    }
}

diesel::table! {
    tag_group_aliases (id) {
        id -> Int4,
        name_id -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    tag_group_members (group_id, tag_id) {
        group_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    tag_groups (id) {
        id -> Int4,
        name_id -> Int4,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    tag_names (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    tag_parents (child_id, parent_id) {
        child_id -> Int4,
        parent_id -> Int4,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name_id -> Int4,
        restricted -> Bool,
        category_id -> Int4,
    }
}

diesel::joinable!(content_file_hash -> content (content_id));
diesel::joinable!(content_files -> content (content_id));
diesel::joinable!(content_files -> storage_roots (storage_root_id));
diesel::joinable!(content_frame_hash -> content (content_id));
diesel::joinable!(content_tag -> content (content_id));
diesel::joinable!(content_tag -> tags (tag_id));
diesel::joinable!(tag_aliases -> tag_names (name_id));
diesel::joinable!(tag_aliases -> tags (tag_id));
diesel::joinable!(tag_group_aliases -> tag_groups (group_id));
diesel::joinable!(tag_group_aliases -> tag_names (name_id));
diesel::joinable!(tag_group_members -> tag_groups (group_id));
diesel::joinable!(tag_group_members -> tags (tag_id));
diesel::joinable!(tag_groups -> tag_names (name_id));
diesel::joinable!(tags -> tag_categories (category_id));
diesel::joinable!(tags -> tag_names (name_id));

diesel::allow_tables_to_appear_in_same_query!(
    content,
    content_file_hash,
    content_files,
    content_frame_hash,
    content_tag,
    storage_roots,
    tag_aliases,
    tag_categories,
    tag_group_aliases,
    tag_group_members,
    tag_groups,
    tag_names,
    tag_parents,
    tags,
);
