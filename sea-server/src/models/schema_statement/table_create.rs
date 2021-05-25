use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SColumnKey {
    NotKey,
    Primary,
    Unique,
    Multiple,
}

impl Default for SColumnKey {
    fn default() -> Self {
        SColumnKey::NotKey
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SColumnExtra {
    pub uuid: bool,
}

impl Default for SColumnExtra {
    fn default() -> Self {
        SColumnExtra { uuid: false }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SColumnType {
    Bool,
    Int,
    Float,
    Double,
    Date,
    Time,
    DateTime,
    Timestamp,
    Char,
    VarChar,
    Text,
    Json,
    Binary,
}

impl Default for SColumnType {
    fn default() -> Self {
        SColumnType::VarChar
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct SColumn {
    pub name: String,
    pub col_type: SColumnType,
    pub null: Option<bool>,
    pub key: Option<SColumnKey>,
    // pub extra: SColumnExtra,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct STable {
    pub name: String,
    pub columns: Vec<SColumn>,
}

#[cfg(test)]
mod test_table {

    use super::*;

    #[test]
    fn convert() {
        let table = STable {
            name: "test".to_string(),
            columns: vec![
                SColumn {
                    name: "id".to_string(),
                    key: Some(SColumnKey::Primary),
                    ..Default::default()
                },
                SColumn {
                    name: "name".to_string(),
                    ..Default::default()
                },
            ],
        };

        let serialized = serde_json::to_string(&table).unwrap();

        let deserialized: STable = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, table);
    }
}
