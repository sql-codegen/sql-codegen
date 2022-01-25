# SQL Code Generator

- [Getting started](#getting-started)
- [Planned features](#planned-features)
- [Ideas](#ideas)
- [Missing parser features](#missing-parser-features)

## Getting started

### Create configuration

First, create the `sql-codegen.json` file in the root of your project.

```json
{
  "dialect": "postgres",
  "connection": {
    "host": "HOST",
    "user": "USER",
    "port": 5432,
    "database": "DATABASE",
    "password": "PASSWORD"
  },
  "schema": "generated/schema.sql",
  "generate": [
    {
      "output": "generated/types.ts",
      "plugins": [
        { "name": "typescript" },
        { "name": "typescript-operations" },
        { "name": "typescript-generic-sdk" },
        { "name": "typescript-pg" }
      ]
    }
  ],
  "queries": "queries/*.sql"
}
```

### Download schema

Next, download schema of your database by executing the following command:

```sh
$ sql-codegen schema
```

Schema file will end up in the `generated/schema.sql` file as specified in the config file.

### Create queries

Let's create some queries in the `queries` directory.

The `queries/find-all-users.sql` file:

```sql
SELECT * FROM users;
```

### Generate SDK code from queries

Run the following command to generate SDK code from the queries.

```sh
sql-codegen
```

The query file name will be used to generate the name of the operation. The `camelCase` name will be used. In this example, the `find-all-users.sql` will be converted to `findAllUsers` function in SDK.

The generated code will be stored in the `generated/types.ts` file.

### Setup SDK

In your TypeScript project, import the `getPgSdk` function from `generated/types.ts` file and initialize it with Postgres client.

```ts
import { Client } from "pg";

const client = new Client({
  user: process.env.POSTGRES_USERNAME!,
  host: process.env.POSTGRES_HOST!,
  database: process.env.POSTGRES_DATABASE!,
  password: process.env.POSTGRES_PASSWORD!,
  port: parseInt(process.env.POSTGRES_PORT!, 10),
});
await client.connect();

const sdk = getPgSdk(client);
```

### Run the code

Now you can run the `findAllUsers` function from the SDK.

```ts
const users = sdk.findAllUsers();
const fullNames = users.map((user) => `${user.first_name} ${user.last_name}`);
```

That's it! You have full autocompletion for your SQL queries. No more typing!

## Planned features

- Support for INSERT/UPDATE/DELETE queries
- Variables/Parameters
- Mapping scalars to custom types in config
- Support for the SQL enum data types
- Returning a single object instead of an array when `LIMIT 1` is used
- Hydration based on `JOIN`s
- Other dialects support like MySQL, MSSQL, SQLLite, etc.
- Multiple queries per file
- Transactions
- Configuring naming convention like `camelCase`, `snake_case`, `PascalCase` etc.
- Plugins generating types and SDK code for Rust, PHP, Python and Java
- Migrations

## Ideas

- Defining aliases for table and column names directly in the schema file

```sql
/* @alias User */
CREATE TABLE users (
  id uuid NOT NULL PRIMARY KEY,
  name text NOT NULL UNIQUE,
  /* @alias firstName */
  first_name character varying(50) NOT NULL
);
```

- Named parameters/variables

Postgres doesn't have named parameters, however they could be deduced from the column name with which the variable is being used. In this example, parameter name would be `id`.

```sql
SELECT *
FROM users
WHERE id = $1
LIMIT 1;
```

It could also be changed using comment.

```sql
/* @alias $1 userId */
SELECT *
FROM users
WHERE id = $1
LIMIT 1;
```

## Missing parser features

Currently there are some missing features in the SQL parser library:

- Variables support
- PostgreSQL array type support
- Comments support
