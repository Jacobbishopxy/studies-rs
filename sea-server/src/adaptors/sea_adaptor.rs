use sea_query::{Alias, ColumnDef, PostgresQueryBuilder, Table, TableCreateStatement};

use crate::models::{SColumn, SColumnKey, SColumnType, STable};

fn column_create(table_create_statement: &mut TableCreateStatement, col: &SColumn) {
    let c = ColumnDef::new(Alias::new(&col.name));

    let c = match col.col_type {
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
    };

    let c = if col.null { c } else { c.not_null() };

    let c = match col.key {
        SColumnKey::NotKey => c,
        SColumnKey::Primary => c.primary_key(),
        SColumnKey::Unique => c.unique_key(),
        SColumnKey::Multiple => c,
    };

    table_create_statement.col(c);
}

pub fn table_create(table: &STable, create_if_not_exists: bool) -> String {
    let table_name = Alias::new(&table.name);

    let mut s = Table::create();
    s.table(table_name);

    if create_if_not_exists {
        s.if_not_exists();
    }

    for c in &table.columns {
        column_create(&mut s, c);
    }

    s.to_string(PostgresQueryBuilder)
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
                    key: SColumnKey::Primary,
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
}
