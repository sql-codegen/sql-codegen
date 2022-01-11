use crate::data;
use sqlparser::ast::{Expr, SelectItem, TableFactor, TableWithJoins};

#[derive(Debug)]
pub struct Projection<'a> {
    pub columns: Vec<ProjectionColumn<'a>>,
}

impl<'a> Projection<'a> {
    pub fn new(
        database: &'a data::Database,
        tables_with_joins: &Vec<TableWithJoins>,
    ) -> Projection<'a> {
        Projection {
            columns: tables_with_joins
                .iter()
                .map(|table_with_joins| match &table_with_joins.relation {
                    TableFactor::Table { name, alias, .. } => {
                        match database.find_table(&name.to_string()) {
                            Some(table) => table
                                .columns
                                .iter()
                                .map(|column| {
                                    ProjectionColumn::new(
                                        database,
                                        match alias {
                                            Some(alias) => alias.name.to_string(),
                                            None => name.to_string(),
                                        },
                                        table,
                                        column.name.clone(),
                                        column,
                                    )
                                })
                                .collect(),
                            None => panic!("Table \"{}\" not found", name),
                        }
                    }
                    _ => vec![],
                })
                .flatten()
                .collect(),
        }
    }

    pub fn project(&'a self, select_items: &Vec<SelectItem>) -> Projection<'a> {
        let projection_columns: Vec<ProjectionColumn> = select_items
            .iter()
            .map(|select_item| match select_item {
                SelectItem::UnnamedExpr(expr) => match expr {
                    Expr::CompoundIdentifier(identifiers) => {
                        println!("------------- CompoundIdentifier = {:?}", identifiers);
                        panic!("The CompoundIdentifier expression is not supported");
                    }
                    Expr::Identifier(identifier) => {
                        panic!("The Identifier expression is not supported");
                        // let column_name = &identifier.value;
                        // let tables_with_identifier: Vec<schema::Table> = self
                        //     .tables
                        //     .iter()
                        //     .filter(|table| table.has_column(column_name))
                        //     .map(|table| {
                        //         let column = table.find_column(column_name).unwrap().clone();
                        //         schema::Table::new(table.name.clone(), vec![column])
                        //     })
                        //     .collect();
                        // if tables_with_identifier.len() > 1 {
                        //     panic!("The identifier \"{}\" is ambiguous", identifier.value);
                        // }
                        // tables_with_identifier
                    }
                    _ => panic!("Not supported expression"),
                },
                SelectItem::ExprWithAlias { .. } => panic!("Not supported expression"),
                SelectItem::QualifiedWildcard(..) => panic!("Not supported expression"),
                SelectItem::Wildcard => self.columns.clone(),
            })
            .flatten()
            .collect();
        Projection {
            columns: projection_columns,
        }
    }

    pub fn size(&self) -> usize {
        self.columns.len()
    }

    pub fn debug(&self) -> () {
        println!("------------------");
        println!("Projection");
        self.columns.iter().for_each(|projection_column| {
            println!(
                "{}.{}.{} -> {}.{}.{}",
                projection_column.database.name,
                projection_column.table.name,
                projection_column.column.name,
                projection_column.database.name,
                projection_column.table_name,
                projection_column.column_name
            );
        });
        println!("------------------");
    }
}

impl<'a> Clone for Projection<'a> {
    fn clone(&self) -> Projection<'a> {
        Projection {
            columns: self.columns.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ProjectionColumn<'a> {
    pub database: &'a data::Database,
    pub table_name: String,
    pub table: &'a data::Table,
    pub column_name: String,
    pub column: &'a data::Column,
}

impl<'a> ProjectionColumn<'a> {
    pub fn new(
        database: &'a data::Database,
        table_name: String,
        table: &'a data::Table,
        column_name: String,
        column: &'a data::Column,
    ) -> ProjectionColumn<'a> {
        ProjectionColumn {
            database,
            table_name,
            table,
            column_name,
            column,
        }
    }
}

impl<'a> Clone for ProjectionColumn<'a> {
    fn clone(&self) -> ProjectionColumn<'a> {
        ProjectionColumn {
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
    use super::*;
    use crate::data;
    use sqlparser::ast::DataType;
    use sqlparser::ast::{Ident, ObjectName, TableAlias, TableFactor};

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

    fn projection_source_to_string(projection: &Projection) -> String {
        projection
            .columns
            .iter()
            .map(|projection| {
                format!(
                    "{}.{}.{}",
                    projection.database.name, projection.table.name, projection.column.name
                )
            })
            .collect::<Vec<String>>()
            .join(",")
    }

    fn projection_target_to_string(projection: &Projection) -> String {
        projection
            .columns
            .iter()
            .map(|projection| {
                format!(
                    "{}.{}.{}",
                    projection.database.name, projection.table_name, projection.column_name
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

        let projection = Projection::new(&public_database, &from_users);

        assert_eq!(projection.size(), 2);
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

        let projection = Projection::new(&public_database, &from_aliased_users);

        assert_eq!(projection.size(), 2);
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
