// @generated automatically by Diesel CLI.

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
        descripton -> Nullable<Text>,
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
    tag_aliases,
    tag_categories,
    tag_group_aliases,
    tag_group_members,
    tag_groups,
    tag_names,
    tag_parents,
    tags,
);
