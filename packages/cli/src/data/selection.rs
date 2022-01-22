use crate::data;

#[derive(Debug)]
pub struct Selection<'a> {
    pub database: &'a data::Database,
    pub table_name: String,
    pub table: &'a data::Table,
    pub column_name: String,
    pub column: &'a data::Column,
}

impl<'a> Selection<'a> {
    pub fn new(
        database: &'a data::Database,
        table_name: String,
        table: &'a data::Table,
        column_name: String,
        column: &'a data::Column,
    ) -> Selection<'a> {
        Selection {
            database,
            table_name,
            table,
            column_name,
            column,
        }
    }
}

impl<'a> Clone for Selection<'a> {
    fn clone(&self) -> Selection<'a> {
        Selection {
            database: self.database,
            table_name: self.table_name.clone(),
            table: self.table,
            column_name: self.column_name.clone(),
            column: self.column,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data;
    use sqlparser::ast::{
        DataType, Ident, ObjectName, SelectItem, TableAlias, TableFactor, TableWithJoins,
    };

    fn create_primary_id_column() -> data::Column {
        data::Column::new(
            "id".to_string(),
            DataType::Int(None),
            true,
            true,
            true,
            None,
        )
    }

    fn create_users_table(alias: Option<String>) -> data::Table {
        let id_column = create_primary_id_column();
        let name_column = data::Column::new(
            "name".to_string(),
            DataType::Varchar(Some(100)),
            false,
            true,
            true,
            None,
        );
        data::Table::new(
            match alias {
                Some(alias) => alias,
                None => "users".to_string(),
            },
            vec![id_column, name_column],
        )
    }

    fn create_comments_table() -> data::Table {
        let id_column = create_primary_id_column();
        data::Table::new("comments".to_string(), vec![id_column])
    }

    fn create_public_database() -> data::Database {
        let users_table = create_users_table(None);
        let comments_table = create_comments_table();

        data::Database::new("public".to_string(), vec![users_table, comments_table])
    }

    fn projection_source_to_string(projection: &data::Projection) -> String {
        projection
            .selections
            .iter()
            .map(|selection| {
                format!(
                    "{}.{}.{}",
                    selection.database.name, selection.table.name, selection.column.name
                )
            })
            .collect::<Vec<String>>()
            .join(",")
    }

    fn projection_target_to_string(projection: &data::Projection) -> String {
        projection
            .selections
            .iter()
            .map(|selection| {
                format!(
                    "{}.{}.{}",
                    selection.database.name, selection.table_name, selection.column_name
                )
            })
            .collect::<Vec<String>>()
            .join(",")
    }

    #[test]
    fn project_table() {
        let public_database = create_public_database();

        let from_users = vec![TableWithJoins {
            relation: TableFactor::Table {
                name: ObjectName(vec![Ident {
                    value: "users".to_string(),
                    quote_style: None,
                }]),
                alias: None,
                args: vec![],
                with_hints: vec![],
            },
            joins: vec![],
        }];

        let select_items = vec![SelectItem::Wildcard];

        let mut projection =
            data::Projection::from_tables_with_joins(&public_database, &from_users);
        projection.filter_by_select_items(&select_items);

        assert_eq!(projection.selections.len(), 2);
        assert_eq!(
            projection_source_to_string(&projection),
            "public.users.id,public.users.name"
        );
        assert_eq!(
            projection_target_to_string(&projection),
            "public.users.id,public.users.name"
        );
    }

    #[test]
    fn project_table_with_alias() {
        let public_database = create_public_database();

        let from_aliased_users = vec![TableWithJoins {
            relation: TableFactor::Table {
                name: ObjectName(vec![Ident {
                    value: "users".to_string(),
                    quote_style: None,
                }]),
                alias: Some(TableAlias {
                    name: Ident {
                        value: "alias".to_string(),
                        quote_style: None,
                    },
                    columns: vec![],
                }),
                args: vec![],
                with_hints: vec![],
            },
            joins: vec![],
        }];

        let select_items = vec![SelectItem::Wildcard];

        let mut projection =
            data::Projection::from_tables_with_joins(&public_database, &from_aliased_users);
        projection.filter_by_select_items(&select_items);

        assert_eq!(projection.selections.len(), 2);
        assert_eq!(
            projection_source_to_string(&projection),
            "public.users.id,public.users.name"
        );
        assert_eq!(
            projection_target_to_string(&projection),
            "public.alias.id,public.alias.name"
        );
    }
}
