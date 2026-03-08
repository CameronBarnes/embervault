use ahash::HashSet;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use super::content::ContentType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    allowed_content_types: HashSet<ContentType>,
}

impl SearchOptions {
    pub fn forbid_content_type(&mut self, content_type: ContentType) -> bool {
        self.allowed_content_types.remove(&content_type)
    }

    pub fn allow_content_type(&mut self, content_type: ContentType) -> bool {
        self.allowed_content_types.insert(content_type)
    }

    pub const fn get_allowed_content_types(&self) -> &HashSet<ContentType> {
        &self.allowed_content_types
    }
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            allowed_content_types: ContentType::iter().collect::<_>(),
        }
    }
}
