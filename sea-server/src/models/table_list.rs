use serde::{Deserialize, Serialize};

use super::STable;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SSchema {
    pub schema: String,
    pub tables: Vec<STable>,
}
