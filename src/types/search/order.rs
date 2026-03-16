use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, IntoStaticStr, VariantArray};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchOrder {
    order: Vec<SortOrderType>,
    favorites_first: bool,
    no_tags_first: bool,
    sort_backwards: bool,
}

impl Default for SearchOrder {
    fn default() -> Self {
        let mut sort_order = Vec::<SortOrderType>::with_capacity(SortOrderType::COUNT);
        sort_order.extend_from_slice(&[
            SortOrderType::Views,
            SortOrderType::ViewTime,
            SortOrderType::Favorite,
        ]);
        Self {
            order: sort_order,
            favorites_first: false,
            no_tags_first: false,
            sort_backwards: false,
        }
    }
}

impl SearchOrder {
    pub fn get_sort_order(&self) -> &[SortOrderType] {
        &self.order
    }

    pub const fn get_sort_order_mut(&mut self) -> &mut Vec<SortOrderType> {
        &mut self.order
    }

    pub const fn favorites_first(&self) -> bool {
        self.favorites_first
    }

    pub const fn set_favorites_first(&mut self, enabled: bool) {
        self.favorites_first = enabled;
    }

    pub const fn set_no_tags_first(&mut self, enabled: bool) {
        self.no_tags_first = enabled;
    }

    pub const fn get_no_tags_first(&self) -> bool {
        self.no_tags_first
    }

    pub const fn should_sort_backwards(&self) -> bool {
        self.sort_backwards
    }

    pub const fn set_sort_backwards(&mut self, enabled: bool) {
        self.sort_backwards = enabled;
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
    DateAdded,
    Favorite,
    Private,
    ContentType,
    HasTags,
    Random,
}
