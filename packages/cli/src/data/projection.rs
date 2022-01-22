use super::Selection;
use crate::{data, duplicated_identifier::DuplicatedIdentifierError};
use sqlparser::ast::{Expr, Ident, SelectItem, TableFactor, TableWithJoins};

#[derive(Debug)]
pub struct Projection<'a> {
    pub selections: Vec<data::Selection<'a>>,
}

impl<'a> Projection<'a> {
    pub fn from_tables_with_joins(
        database: &'a data::Database,
        tables_with_joins: &Vec<TableWithJoins>,
    ) -> Projection<'a> {
        let selections = tables_with_joins
            .iter()
            .map(|table_with_joins| match &table_with_joins.relation {
                TableFactor::Table { name, alias, .. } => {
                    match database.find_table(&name.to_string()) {
                        Some(table) => table
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
                            .collect(),
                        None => panic!("Table \"{}\" not found", name),
                    }
                }
                _ => vec![],
            })
            .flatten()
            .collect::<Vec<data::Selection>>();

        Projection { selections }
    }

    fn filter_by_compound_identifier(
        &self,
        identifiers: &Vec<Ident>,
        alias: Option<&Ident>,
    ) -> Vec<Selection<'a>> {
        if identifiers.len() != 2 {
            let compound_identifier = identifiers
                .iter()
                .map(|identifier| identifier.value.clone())
                .collect::<Vec<String>>()
                .join(".");
            panic!(
                "The \"{compound_identifier}\" compound identifier expression is not supported",
                compound_identifier = compound_identifier
            );
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
            panic!("Column \"{}.{}\" does not exist", table_name, column_name);
        }
        if filtered_selections.len() > 1 {
            panic!(
                "Column reference \"{}.{}\" is ambiguous",
                table_name, column_name
            );
        }
        if let Some(alias) = alias {
            let mut selection = filtered_selections.first().unwrap().clone();
            selection.column_name = alias.value.clone();
            return vec![selection];
        }
        filtered_selections
    }

    fn filter_by_identifier(
        &self,
        identifier: &Ident,
        alias: Option<&Ident>,
    ) -> Vec<Selection<'a>> {
        let filtered_selections = self
            .selections
            .iter()
            .cloned()
            .filter(|selection| selection.column.name == identifier.value)
            .collect::<Vec<Selection>>();
        if filtered_selections.len() == 0 {
            panic!("Column \"{}\" does not exist", identifier.value);
        }
        if filtered_selections.len() > 1 {
            panic!("Column reference \"{}\" is ambiguous", identifier.value);
        }
        if let Some(alias) = alias {
            let mut selection = filtered_selections.first().unwrap().clone();
            selection.column_name = alias.value.clone();
            return vec![selection];
        }
        filtered_selections
    }

    fn find_duplicated_selections(&self) -> Option<DuplicatedIdentifierError> {
        for selection_a in &self.selections {
            for selection_b in &self.selections {
                if selection_a.column != selection_b.column
                    && selection_a.column_name == selection_b.column_name
                {
                    return Some(DuplicatedIdentifierError::new(
                        selection_a.column_name.clone(),
                    ));
                }
            }
        }
        None
    }

    pub fn filter_by_select_items(&mut self, select_items: &Vec<SelectItem>) {
        let selections = select_items
            .iter()
            .map(|select_item| match select_item {
                SelectItem::UnnamedExpr(expr) => match expr {
                    Expr::CompoundIdentifier(identifiers) => {
                        self.filter_by_compound_identifier(identifiers, None)
                    }
                    Expr::Identifier(identifier) => self.filter_by_identifier(identifier, None),
                    _ => panic!("Not supported expression"),
                },
                SelectItem::ExprWithAlias { expr, alias } => match expr {
                    Expr::CompoundIdentifier(identifiers) => {
                        self.filter_by_compound_identifier(identifiers, Some(alias))
                    }
                    Expr::Identifier(identifier) => {
                        self.filter_by_identifier(identifier, Some(alias))
                    }
                    _ => panic!("Not supported expression"),
                },
                SelectItem::QualifiedWildcard(..) => {
                    panic!("\"{}\" is not supported expression", select_item)
                }
                SelectItem::Wildcard => self.selections.clone(),
            })
            .flatten()
            .collect::<Vec<data::Selection>>();

        if let Some(error) = self.find_duplicated_selections() {
            panic!("Duplicated identifier \"{}\"", error.identifier);
        }

        self.selections = selections;
    }
}
