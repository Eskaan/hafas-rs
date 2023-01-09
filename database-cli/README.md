# Database-cli toolchain
This is a command line interface using the [`hafas_wrap`] library.

## Use and Usage
The current use is to create a database of trips, train_types, locations and operators. 
For the exact layout, see `database_layout.png` at the root of the project.
You can also create a simple diagram from usage statistics.

Altrough the program does not need a live database to compile, it will need a postgres database to function.
The database needs to be on localhost, with the name `db-statistics` and the user `postgres` (password-less) 
being able to access all tables specified in the database layout. 
To prepare the database for use, run `database-cli migrate`. This will create all neccecary schemas, tables, types and functions.

For the exact usage, refer to the rustdoc dcumentation of the [`main`] method or the usage section in the book.

# Installation
Please refer to [`docs/src/installation.md`](./docs/src/installation.md)