use anyhow::Result;
use diesel::deserialize::Queryable;
use diesel::{dsl::count_star, prelude::*};

use crate::db::{
    DbPool,
    schema::{content_tag, tag_categories, tag_names, tag_parents, tags},
};

#[derive(Debug, Clone, Queryable)]
pub struct TagRecord {
    pub id: i32,
    pub name: String,
    pub restricted: bool,
    pub category_id: i32,
}

#[derive(Debug, Clone, Queryable)]
pub struct TagParentRecord {
    pub child_id: i32,
    pub parent_id: i32,
}

#[derive(Debug, Clone, Queryable)]
pub struct TagUsageRecord {
    pub tag_id: i32,
    pub usage_count: i64,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::tag_categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TagCategory {
    id: i32,
    name: String,
    description: Option<String>,
    permanent: bool,
}

#[derive(Debug, Clone)]
pub struct TagTreeData {
    pub tags: Vec<TagRecord>,
    pub parents: Vec<TagParentRecord>,
    pub categories: Vec<TagCategory>,
    pub usage: Vec<TagUsageRecord>,
}

pub fn load_tag_tree_data(pool: &DbPool) -> Result<TagTreeData> {
    let mut conn = pool.get()?;

    let tags = tags::table
        .inner_join(tag_names::table)
        .select((
            tags::id,
            tag_names::name,
            tags::restricted,
            tags::category_id,
        ))
        .load::<TagRecord>(&mut conn)?;

    let parents = tag_parents::table
        .select((tag_parents::child_id, tag_parents::parent_id))
        .load::<TagParentRecord>(&mut conn)?;

    let categories = tag_categories::table
        .select(TagCategory::as_select())
        .load::<TagCategory>(&mut conn)?;

    let usage = content_tag::table
        .group_by(content_tag::tag_id)
        .select((content_tag::tag_id, count_star()))
        .load::<TagUsageRecord>(&mut conn)?;

    Ok(TagTreeData {
        tags,
        parents,
        categories,
        usage,
    })
}
