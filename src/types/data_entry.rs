use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DataEntry {
    pub text: String,
    pub created: String,
    pub views: u32,
    pub likes: u32,
}
