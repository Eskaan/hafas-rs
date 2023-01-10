# Database-cli toolchain
This is a command line interface using the [`hafas_wrap`] library.

## Use and Usage
The current use is to create a database of trips, train types, locations and operators. 
For the exact layout, see `database_layout.png` at the root of the project.
You can also create a simple diagram from usage statistics.

Although the program does not need a live database to compile, it will need a postgres database to function.
The database has to be on localhost, with the name `db-statistics` and the user `postgres` (password-less) 
being able to access all tables specified in the database layout. 
To prepare the database for use, run `database-cli migrate`. This will create all necessary schemas, tables, types and functions.

For the exact usage, refer to the rustdoc documentation of the [`main`] method or the usage section in the book.

# Installation
Please refer to the [`installation section`](/book/installation.html)