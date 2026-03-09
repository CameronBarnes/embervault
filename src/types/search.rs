use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, IntoStaticStr, VariantArray};

use super::content;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Options {
    allowed_content_types: Vec<content::Type>,
    sort_order: Vec<SortOrderType>,
    search_type: r#Type,
    favorites_first: bool,
    allow_private: bool,
    sort_backwards: bool,
}

impl Options {
    pub fn forbid_content_type(&mut self, content_type: content::Type) {
        self.allowed_content_types
            .retain(|item| *item != content_type);
    }

    pub fn allow_content_type(&mut self, content_type: content::Type) {
        if self.allowed_content_types.len() == content::Type::COUNT
            || self.allowed_content_types.contains(&content_type)
        {
            return;
        }
        self.allowed_content_types.push(content_type);
    }

    pub fn set_content_type_status(&mut self, content_type: content::Type, allowed: bool) {
        if allowed {
            self.allow_content_type(content_type);
        } else {
            self.forbid_content_type(content_type);
        }
    }

    pub fn get_allowed_content_types(&self) -> &[content::Type] {
        &self.allowed_content_types
    }

    pub const fn has_any_allowed_content_types(&self) -> bool {
        !self.allowed_content_types.is_empty()
    }

    pub fn get_sort_order(&self) -> &[SortOrderType] {
        &self.sort_order
    }

    pub const fn get_sort_order_mut(&mut self) -> &mut Vec<SortOrderType> {
        &mut self.sort_order
    }

    pub const fn favorites_first(&self) -> bool {
        self.favorites_first
    }

    pub const fn allow_private(&self) -> bool {
        self.allow_private
    }

    pub const fn set_favorites_first(&mut self, enabled: bool) {
        self.favorites_first = enabled;
    }

    pub const fn set_allow_private(&mut self, enabled: bool) {
        self.allow_private = enabled;
    }

    pub const fn should_sort_backwards(&self) -> bool {
        self.sort_backwards
    }

    pub const fn set_sort_backwards(&mut self, enabled: bool) {
        self.sort_backwards = enabled;
    }

    #[must_use]
    pub const fn search_type(&self) -> &Type {
        &self.search_type
    }

    #[must_use]
    pub const fn search_type_mut(&mut self) -> &mut Type {
        &mut self.search_type
    }

    pub const fn set_search_type(&mut self, search_type: Type) {
        self.search_type = search_type;
    }
}

impl Default for Options {
    fn default() -> Self {
        let mut sort_order = Vec::<SortOrderType>::with_capacity(SortOrderType::COUNT);
        sort_order.extend_from_slice(&[
            SortOrderType::Views,
            SortOrderType::ViewTime,
            SortOrderType::Favorite,
        ]);
        Self {
            allowed_content_types: content::Type::VARIANTS.into(),
            sort_order,
            search_type: Type::default(),
            favorites_first: false,
            allow_private: false,
            sort_backwards: false,
        }
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    IntoStaticStr,
    Display,
    VariantArray,
    EnumIs,
    EnumIter,
    EnumCount,
    PartialEq,
    Eq,
    Hash,
)]
pub enum Type {
    #[default]
    Content,
    Pool,
    Both,
    None,
}

impl Type {
    pub const fn content(self) -> bool {
        matches!(self, Self::Content | Self::Both)
    }

    pub const fn pool(self) -> bool {
        matches!(self, Self::Pool | Self::Both)
    }

    #[must_use]
    pub const fn set_content(self, enabled: bool) -> Self {
        if enabled {
            if self.is_content() || self.is_none() {
                Self::Content
            } else {
                Self::Both
            }
        } else {
            match self {
                Self::Content => Self::None,
                Self::Both => Self::Pool,
                Self::Pool | Self::None => self,
            }
        }
    }

    pub const fn update_content(&mut self, enabled: bool) {
        *self = self.set_content(enabled);
    }

    #[must_use]
    pub const fn set_pool(self, enabled: bool) -> Self {
        if enabled {
            if self.is_pool() || self.is_none() {
                Self::Pool
            } else {
                Self::Both
            }
        } else {
            match self {
                Self::Pool => Self::None,
                Self::Both => Self::Content,
                Self::Content | Self::None => self,
            }
        }
    }

    pub const fn update_pool(&mut self, enabled: bool) {
        *self = self.set_pool(enabled);
    }

    pub const fn is_some(self) -> bool {
        !self.is_none()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    IntoStaticStr,
    Display,
    VariantArray,
    EnumIs,
    EnumIter,
    EnumCount,
    PartialEq,
    Eq,
    Hash,
)]
pub enum SortOrderType {
    Views,
    ViewTime,
    Favorite,
    Private,
    ContentType,
    Random,
}
