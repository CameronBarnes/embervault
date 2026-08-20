use crate::db::{DbPool, schema::content};
use diesel::prelude::*;
use anyhow::Result;

pub fn content_count(pool: &DbPool) -> Result<i64> {
    let mut conn = pool.get()?;

    let count = content::table.count().get_result(&mut conn)?;
    Ok(count)
}
