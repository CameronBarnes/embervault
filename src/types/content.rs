use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, IntoStaticStr, VariantArray};

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
pub enum Type {
    Image,
    Video,
    Gif,
    Text,
    Other,
}
