use serde::{Deserialize, Serialize};
use strum::{Display, EnumIs, EnumIter, IntoStaticStr, VariantArray};

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
    PartialEq,
    Eq,
    Hash,
)]
pub enum ContentType {
    Image,
    Video,
    Gif,
    Text,
    Other,
}
