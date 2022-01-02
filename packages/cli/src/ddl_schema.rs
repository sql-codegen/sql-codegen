use crate::config;
use postgres::{Client, NoTls, Row};
use std::fs;

pub struct DdlSchema {
    client: Client,
}

impl DdlSchema {
    pub fn new(config: &config::Config) -> Self {
        let params = format!(
            "host={host} user={user} port={port} dbname={database} password={password}",
            host = config.connection.host,
            user = config.connection.user,
            port = config.connection.port,
            database = config.connection.database,
            password = config.connection.password
        );
        DdlSchema {
            client: Client::connect(&params, NoTls).expect("Could not connect to postgres"),
        }
    }

    fn get_column_ddl(&self, row: &Row) -> String {
        let column_name: &str = row.get("column_name");
        let column_type = row
            .get::<_, &str>("column_type")
            .to_uppercase()
            // TODO: Currently parser doesn't support array types.
            .replace("[]", "");
        let is_primary_key: bool = row.get("primary_key");
        let is_unique: bool = row.get("unique");
        let is_not_null: bool = row.get("not_null");
        let default_value: Option<String> = row.get("default");

        let mut column_options: Vec<&str> = vec![&column_type];
        if is_not_null {
            column_options.push("NOT NULL");
        }
        if is_unique {
            column_options.push("UNIQUE");
        }
        if is_primary_key {
            column_options.push("PRIMARY KEY");
        }
        format!(
            "\t\"{name}\" {options}",
            name = column_name,
            options = column_options.join(" ")
        )
    }

    fn get_create_table_opening_ddl(&mut self, table_name: &str) -> String {
        format!("CREATE TABLE \"{table_name}\" (\n", table_name = table_name)
    }

    fn get_create_table_closing_ddl(&mut self) -> &str {
        "\n);\n"
    }

    pub fn generate(&mut self) -> () {
        let mut ddl = String::from("");

        let rows = self
            .client
            .query(TABLES_QUERY, &[])
            .expect("Could not query list of tables");

        let mut prev_row: Option<Row> = None;
        for row in rows {
            let table_name = row.get::<_, &str>("table_name");
            if let None = &prev_row {
                ddl.push_str(&self.get_create_table_opening_ddl(table_name));
            } else if let Some(prev_row) = &prev_row {
                let prev_table_name = prev_row.get::<&str, String>("table_name");
                if prev_table_name != table_name {
                    ddl.push_str(self.get_create_table_closing_ddl());
                    ddl.push_str("\n");
                    ddl.push_str(&self.get_create_table_opening_ddl(table_name));
                } else {
                    ddl.push_str(",\n");
                }
            }
            ddl.push_str(&self.get_column_ddl(&row));
            prev_row = Some(row);
        }
        if ddl.len() > 0 {
            ddl.push_str(self.get_create_table_closing_ddl());
        }

        fs::create_dir_all("generated").expect("Error creating directory");
        fs::write("generated/schema.sql", ddl).expect("Error creating schema.gql file");
    }
}

const TABLES_QUERY: &str =
"SELECT
  pg_class.relname AS table_name,
  pg_attribute.attnum AS column_number,
  pg_attribute.attname AS column_name,
  pg_catalog.format_type(pg_attribute.atttypid, pg_attribute.atttypmod) AS column_type,
  COALESCE(pg_constraint.contype = 'p', FALSE) AS primary_key,
  COALESCE(pg_constraint.contype = 'u', FALSE) AS unique,
  pg_attribute.attnotnull AS not_null,
  CASE WHEN pg_constraint.contype = 'f' THEN pg_class2.relname END AS foreign_key,
  CASE WHEN pg_constraint.contype = 'f' THEN pg_constraint.confkey END AS foreign_key_fieldnum,
  CASE WHEN pg_constraint.contype = 'f' THEN pg_constraint.conkey END AS foreign_key_connnum,
  CASE WHEN pg_attribute.atthasdef = 't' THEN pg_attrdef.adsrc END AS default
FROM
  pg_attribute
  JOIN pg_class ON pg_class.oid = pg_attribute.attrelid
  JOIN pg_type ON pg_type.oid = pg_attribute.atttypid
  LEFT JOIN pg_attrdef ON pg_attrdef.adrelid = pg_class.oid AND pg_attrdef.adnum = pg_attribute.attnum
  LEFT JOIN pg_namespace ON pg_namespace.oid = pg_class.relnamespace
  LEFT JOIN pg_constraint ON pg_constraint.conrelid = pg_class.oid AND pg_attribute.attnum = ANY (pg_constraint.conkey)
  LEFT JOIN pg_class AS pg_class2 ON pg_constraint.confrelid = pg_class2.oid
WHERE
  pg_class.relkind = 'r'::CHAR
  AND pg_namespace.nspname = 'public'
  AND pg_attribute.attnum > 0
ORDER BY
  table_name ASC,
  column_number ASC;";
