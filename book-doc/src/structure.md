# Project structure
## General structure
The project is split into two main parts: 
- [hafas-wrap], the library to access the HAFAS[^hafas].
  
  It provides simple ways to access the HAFAS through multiple methods, 
  and a generic type system which also supports custom request and response types out of the common usage.

- [database-cli] that uses data queried through the library and injects certain parts into a postgresql database for further processig. 
  
  It also (currently) contains the subcommands to print a diagram showing the most used stations in the datbase. 
  **If you are searching about how to use this project, you will have to look here.**

Most of the project, althrough not all, is written in a more procedual style, as this program is based on a cli and corresponding library. 
There are more object oriented parts in the library.

## Other documentation
Each of those projects contain their own `README.md` file further explaining their use.

I also recommend using the rust-doc documentation for the project-level documentation, found in the `docs` folder [`./docs/database_cli/index.html`](/docs/database_cli/index.html).

[hafas-wrap]: ./hafas-wrap.html
[database-cli]: ./database-cli.html
[^hafas]: Hacon Fahrplan Auskunfts System
