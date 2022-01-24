use super::Selection;
use crate::{data, error};
use sqlparser::ast::{Expr, Ident, SelectItem, TableFactor, TableWithJoins};

#[derive(Debug)]
pub struct Projection<'a> {
    pub selections: Vec<data::Selection<'a>>,
}

impl<'a> Projection<'a> {
    pub fn from_tables_with_joins(
        database: &'a data::Database,
        tables_with_joins: &Vec<TableWithJoins>,
    ) -> Result<Projection<'a>, error::CodegenError> {
        let selections_of_selections = tables_with_joins
            .iter()
            .map(|table_with_joins| match &table_with_joins.relation {
                TableFactor::Table { name, alias, .. } => {
                    match database.find_table(&name.to_string()) {
                        Some(table) => {
                            let selections = table
                                .columns
                                .iter()
                                .map(|column| {
                                    data::Selection::new(
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
                                .collect::<Vec<data::Selection>>();
                            Ok(selections)
                        }
                        None => Err(error::CodegenError::QueryError(format!(
                            "Table \"{name}\" not found"
                        ))),
                    }
                }
                _ => Ok(vec![]),
            })
            .collect::<Result<Vec<Vec<data::Selection>>, error::CodegenError>>()?;

        let selections = selections_of_selections
            .into_iter()
            .flatten()
            .collect::<Vec<Selection>>();

        Ok(Projection { selections })
    }

    fn filter_by_compound_identifier(
        &self,
        identifiers: &Vec<Ident>,
        alias: Option<&Ident>,
    ) -> Result<Vec<Selection<'a>>, error::CodegenError> {
        if identifiers.len() != 2 {
            let compound_identifier = identifiers
                .iter()
                .map(|identifier| identifier.value.clone())
                .collect::<Vec<String>>()
                .join(".");
            return Err(error::CodegenError::QueryError(format!(
                "The \"{compound_identifier}\" compound identifier expression is not supported",
            )));
        }
        let table_name = identifiers[0].value.clone();
        let column_name = identifiers[1].value.clone();
        let filtered_selections = self
            .selections
            .iter()
            .cloned()
            .filter(|selection| {
                selection.table.name == table_name && selection.column.name == column_name
            })
            .collect::<Vec<Selection>>();
        if filtered_selections.len() == 0 {
            return Err(error::CodegenError::QueryError(format!(
                "Column \"{table_name}.{column_name}\" does not exist"
            )));
        }
        if filtered_selections.len() > 1 {
            return Err(error::CodegenError::QueryError(format!(
                "Column reference \"{table_name}.{column_name}\" is ambiguous",
            )));
        }
        if let Some(alias) = alias {
            let mut selection = filtered_selections.first().unwrap().clone();
            selection.column_name = alias.value.clone();
            return Ok(vec![selection]);
        }
        Ok(filtered_selections)
    }

    fn filter_by_identifier(
        &self,
        identifier: &Ident,
        alias: Option<&Ident>,
    ) -> Result<Vec<Selection<'a>>, error::CodegenError> {
        let filtered_selections = self
            .selections
            .iter()
            .cloned()
            .filter(|selection| selection.column.name == identifier.value)
            .collect::<Vec<Selection>>();
        if filtered_selections.len() == 0 {
            return Err(error::CodegenError::QueryError(format!(
                "Column \"{}\" does not exist",
                identifier.value
            )));
        }
        if filtered_selections.len() > 1 {
            return Err(error::CodegenError::QueryError(format!(
                "Column reference \"{}\" is ambiguous",
                identifier.value
            )));
        }
        if let Some(alias) = alias {
            let mut selection = filtered_selections.first().unwrap().clone();
            selection.column_name = alias.value.clone();
            return Ok(vec![selection]);
        }
        Ok(filtered_selections)
    }

    pub fn filter_by_select_items(
        &mut self,
        select_items: &Vec<SelectItem>,
    ) -> Result<(), error::CodegenError> {
        let selections_of_selections = select_items
            .iter()
            .map(|select_item| match select_item {
                SelectItem::UnnamedExpr(expr) => match expr {
                    Expr::CompoundIdentifier(identifiers) => {
                        self.filter_by_compound_identifier(identifiers, None)
                    }
                    Expr::Identifier(identifier) => self.filter_by_identifier(identifier, None),
                    _ => Err(error::CodegenError::QueryError(format!(
                        "Not supported expression"
                    ))),
                },
                SelectItem::ExprWithAlias { expr, alias } => match expr {
                    Expr::CompoundIdentifier(identifiers) => {
                        self.filter_by_compound_identifier(identifiers, Some(alias))
                    }
                    Expr::Identifier(identifier) => {
                        self.filter_by_identifier(identifier, Some(alias))
                    }
                    _ => Err(error::CodegenError::QueryError(format!(
                        "Not supported expression"
                    ))),
                },
                SelectItem::QualifiedWildcard(..) => Err(error::CodegenError::QueryError(format!(
                    "\"{select_item}\" is not supported expression"
                ))),
                SelectItem::Wildcard => Ok(self.selections.clone()),
            })
            .collect::<Result<Vec<Vec<data::Selection>>, error::CodegenError>>()?;

        let selections = selections_of_selections
            .into_iter()
            .flatten()
            .collect::<Vec<Selection>>();

        self.selections = selections;

        Ok(())
    }
}
