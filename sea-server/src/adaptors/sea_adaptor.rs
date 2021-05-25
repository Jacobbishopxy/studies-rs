use sea_query::{Alias, ColumnDef, PostgresQueryBuilder, Table};

use crate::models::{SColumn, SColumnAlterCase, SColumnKey, SColumnType, STable, STableAlter};

fn column_type_grant(c: ColumnDef, col_type: &SColumnType) -> ColumnDef {
    match col_type {
        SColumnType::Binary => c.binary(),
        SColumnType::Bool => c.boolean(),
        SColumnType::Int => c.integer(),
        SColumnType::Float => c.float(),
        SColumnType::Double => c.double(),
        SColumnType::Date => c.date(),
        SColumnType::Time => c.time(),
        SColumnType::DateTime => c.date_time(),
        SColumnType::Timestamp => c.timestamp(),
        SColumnType::Char => c.char(),
        SColumnType::VarChar => c.string(),
        SColumnType::Text => c.text(),
        SColumnType::Json => c.json(),
    }
}

fn column_create(col: &SColumn) -> ColumnDef {
    let c = ColumnDef::new(Alias::new(&col.name));
    let c = column_type_grant(c, &col.col_type);
    let c = if col.null.unwrap_or(true) == true {
        c
    } else {
        c.not_null()
    };
    let c = if let Some(ck) = &col.key {
        match ck {
            SColumnKey::NotKey => c,
            SColumnKey::Primary => c.primary_key(),
            SColumnKey::Unique => c.unique_key(),
            SColumnKey::Multiple => c,
        }
    } else {
        c
    };

    c
}

pub fn table_create(table: &STable, create_if_not_exists: bool) -> String {
    let mut s = Table::create();
    s.table(Alias::new(&table.name));

    if create_if_not_exists {
        s.if_not_exists();
    }

    for c in &table.columns {
        s.col(column_create(c));
    }

    s.to_string(PostgresQueryBuilder)
}

pub fn table_alter(alter: &STableAlter) -> Vec<String> {
    let s = Table::alter().table(Alias::new(&alter.name));
    let mut alter_series = vec![];

    for a in &alter.alter {
        match a {
            SColumnAlterCase::Add(c) => {
                alter_series.push(s.clone().add_column(column_create(c)));
            }
            SColumnAlterCase::Modify(c) => {
                alter_series.push(s.clone().modify_column(column_create(c)));
            }
            SColumnAlterCase::Rename(c) => {
                let from_name = Alias::new(&c.from_name);
                let to_name = Alias::new(&c.to_name);
                alter_series.push(s.clone().rename_column(from_name, to_name));
            }
            SColumnAlterCase::Drop(c) => {
                alter_series.push(s.clone().drop_column(Alias::new(&c.name)));
            }
        }
    }

    alter_series
        .iter()
        .map(|a| a.to_string(PostgresQueryBuilder))
        .collect()
}

#[cfg(test)]
mod tests_sea_adaptor {
    use super::*;

    #[test]
    fn test_table_create() {
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

        println!("{:?}", table_create(&table, true));
    }

    #[test]
    fn test_table_alter() {
        let alter = STableAlter {
            name: "test".to_string(),
            alter: vec![SColumnAlterCase::Add(SColumn {
                name: "name".to_string(),
                ..Default::default()
            })],
        };

        println!("{:?}", table_alter(&alter));
    }
}
