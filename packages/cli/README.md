# SQL Code Generator

## Configuration

In the config file you can define naming convention of the generated `schema.gql` file as well as for your queries.

```json
{
  "connection": {
    "host": "HOST",
    "user": "USER",
    "port": 5432,
    "database": "DATABASE",
    "password": "PASSWORD"
  },
  "schema": "./schema.gql",
  "queries": "./queries/*.gql",
  "namingConvention": {
    "table": "PascalCase",
    "column": "lowerCase",
  }
}
```

## Generating schema

```sh
$ sql-codegen generate schema
```

You can also define your mapping directly inside the `schema.sql` file.

```sql
/* @alias User */
CREATE TABLE users (
  id uuid NOT NULL PRIMARY KEY,
  name text NOT NULL UNIQUE,
  /* @alias firstName */
  first_name character varying(50) NOT NULL
);
```

## Generating code from queries

The following SQL query in the `get-user-by-id.sql` file:

```sql
-- If the @paramName comment is not defined then name of the params will be taken from the column name with which it's being used.
/* @paramName $1 id */
-- Your custom alias always has a precedence
SELECT *, name AS "another_name"
FROM users
WHERE id = $1
LIMIT 1;
```

After running the command:

```sh
$ sql-codegen generate queries
```

It will product the following function and types:

```ts
type User {
  firstName: string;
  id: Uuid;
  name: string;
  another_name: string;
};

type GetUserByIdParams = {
  id: Uuid;
};

type GetUserById = (params: GetUserByIdParams) => User;

type SdkFactoryDeps = {
  pgClient: PgClient;
}

export const GetUserByIdQuery = 'SELECT first_name AS "firstName", id AS "id", name AS "name" FROM users AS "User" WHERE id = $1 LIMIT 1;';

const sqkFactory = (deps: SdkFactoryDeps) => {
  const getUserById: GetUserById = (params) => {
    const result = await pgClient.query(GetUserByIdQuery, [params.id]);
    return result.rows[0];
  };

  return { getUserById };
}
```

Which you can use as follows:

```ts
const sdk = sdkFactory({ pgClient });
const user = await sdk.getUserById({ id: 1 });
```

## Missing Features

Currently there are some missing features in the SQL parser library:
- Variables support
- PostgreSQL array types support
- Comments support

## Tasks

1. Create structure for taking CLI commands.
2. Process `generate schema` command that generates `schema.sql` file from the PostgreSQL database connection options.
3. Process `generate queries` command that reads all the queries from the SQL files.
4. Write tests to run CLI commands.
5. Function taking AST of the query and generating projection.
6. Function converting query projection to TypeScript types and generating SDK code.
7. Function converting schema projection to TypeScript types.
8. Utility functions for reading and writing files to disk.
