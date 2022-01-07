# Flow

## Source data

1. Analyze an expression inside the `FROM` clause of a SQL query.
2. Construct the source data schema from the `FROM` clause which might be missing but we still need to generate an empty schema, to be used for projection. Schema should be a vector of objects like:

```rs
pub struct SourceField {
  pub table_name: Option<String>; // Table name if present
  pub name: String; // Column name in table or custom field name
  pub sql_type: String; // SQL type like varchar or uuid
}

let source_data: Vec<SourceField>;
```

3. When the data is loaded from a table then we have to collect all the table columns and their types. If there are joins to other tables, then we have to load them as well and just keep adding new columns to the schema vector. We should keep name of a table, so that we can distinguish columns with the same name. Even though there can be two columns with the same name, JavaScript does not allow object with duplicated properties (`{ a: "value1", a: "value2"}`), so it will be overridden by the last value (`{ a: "value2"}`). When the source data is constructed dynamically, we still know field name and a SQL type.

## Plugins

### TypeScript

The base type name for a table will not only be used when creating base TypeScript types. It will also be used in the SDK code when query selects all the fields from the table. In such a case we can set the return type to the base type.
```sql
SELECT * FROM user;
```
```ts
type Result = User;
```
If a query selects some subset of columns, then we can use base type for the return type in the following way.
```sql
SELECT id, name FROM user;
```
```ts
type Result = Pick<User, "id" | "name">
```
We should reuse base type as much as possible. The only time when we shouldn't use it or should use it partially is when user aliased columns.
```sql
SELECT id, name AS "fullName" FROM user;
```
```ts
type Result = Pick<User, "id"> & { fullName: string };
```