use serde::{Deserialize, Serialize};

use super::super::SColumn;

pub type SColumnAdd = SColumn;

pub type SColumnModify = SColumn;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SColumnRename {
    pub from_name: String,
    pub to_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SColumnDrop {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SColumnAlterCase {
    Add(SColumnAdd),
    Modify(SColumnModify),
    Rename(SColumnRename),
    Drop(SColumnDrop),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct STableAlter {
    pub name: String,
    pub alter: Vec<SColumnAlterCase>,
}
