use sqlparser::ast::{ColumnDef, ColumnOption, DataType};

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub sql_type: DataType,
    pub is_primary_key: bool,
    pub is_unique: bool,
    pub is_not_null: bool,
    pub default_value: Option<String>,
}

impl Clone for Column {
    fn clone(&self) -> Column {
        Column {
            name: self.name.clone(),
            sql_type: self.sql_type.clone(),
            is_primary_key: self.is_primary_key,
            is_unique: self.is_unique,
            is_not_null: self.is_not_null,
            default_value: self.default_value.clone(),
        }
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Column) -> bool {
        self.name == other.name
            && self.sql_type == other.sql_type
            && self.is_primary_key == other.is_primary_key
            && self.is_unique == other.is_unique
            && self.is_not_null == other.is_not_null
            && self.default_value == other.default_value
    }
}

impl Column {
    pub fn new(
        name: String,
        sql_type: DataType,
        is_primary_key: bool,
        is_unique: bool,
        is_not_null: bool,
        default_value: Option<String>,
    ) -> Column {
        Column {
            name,
            sql_type,
            is_primary_key,
            is_unique,
            is_not_null,
            default_value,
        }
    }

    pub fn from_column_definition(column_definition: &ColumnDef) -> Column {
        let is_not_null = column_definition
            .options
            .iter()
            .any(|column| column.option == ColumnOption::NotNull);
        let is_unique = column_definition
            .options
            .iter()
            .any(|column| match column.option {
                ColumnOption::Unique { .. } => true,
                _ => false,
            });
        let is_primary_key = column_definition
            .options
            .iter()
            .any(|column| match column.option {
                ColumnOption::Unique { is_primary } => is_primary,
                _ => false,
            });
        let default_value =
            column_definition
                .options
                .iter()
                .find_map(|column| match column.option {
                    ColumnOption::Default { .. } => Some("DEFAULT".to_string()),
                    _ => None,
                });
        Column::new(
            column_definition.name.value.clone(),
            column_definition.data_type.clone(),
            is_primary_key,
            is_unique,
            is_not_null,
            default_value,
        )
    }
}
