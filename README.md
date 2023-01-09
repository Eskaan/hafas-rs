# About the project
This project was done for a high school presentation by me, Adrian Struwe 
([Eskaan]). It is currently licenced under the [GPL3] licence.

## Inspiration and Credit
The idea for this project is not my own. This has ben done before by [David Kiesel] and was presented by him in the [C36C3] Congress. Credit also goes to [Marudor] and his [bahn.expert] project, it was very helpful for me to understand the inner workings of HAFAS.

# Project structure
The project is split into two main parts: 
- [`hafas_wrap`], the library to access the HAFAS (Hacon Fahrplan Auskunfts System).
  This library contains the request and response data structures and a request system.
- [`database_cli`](crate) that uses data queried through the library and injects certain parts into a postgresql database for further processig. It also (currently) contains the subcommands to print a diagram showing the most used stations in the datbase. **If you are searching about how to use this project, you will have to look here.**

Each of those projects contain their own `README.md` file further explaining their use.

I also recommend using the rust-doc documentation for the project-level documentation, found in the `doc` folder (`./doc/database_cli/index.html`).

# Goals
The formulated goal for this project was to query and compare train schedule to realtime data. Sadly, after many experiments I realized this API was designed to resist such data collection attempts. I am still trying to find a feasible way (that does not overload the API) to collect such data.
As I could not finish with my targeted goal, I insted selected one more realistic; to display the most used stations in the database.

# Future of the project
I plan on continuing to work on this and maybe one day make a good HAFAS library for Rust. This is the primary goal for the future, after which I plan to create a public API that serves better readable responses than HAFAS.

[Eskaan]: https://github.com/eskaan
[GPL3]: https://www.gnu.org/licenses/gpl-3.0.txt
[David Kiesel]: https://www.dkriesel.com/
[C36C3]: https://events.ccc.de/congress/2019/
[Marudor]: https://github.com/marudor
[bahn.expert]: https://bahn.expert
[hafas-rs]: ./hafas-wrap/README.md
[database-cli]: ./database-cli/README.md