# About the project
This project was done for a high school presentation (GFS) by me, Adrian Struwe 
([Eskaan]). 

It is currently licenced under the [GPL3] licence, allowing open distribution and modification of the code.

The project is fully documented, both in source code (per [rustdoc]) as in this [mdbook].

[Eskaan]: https://github.com/eskaan
[GPL3]: https://www.gnu.org/licenses/gpl-3.0.txt
[rustdoc]: doc.rust-lang.org/rustdoc
[mdbook]: rust-lang.github.io/mdBook

## Progress

I initially started working on the idea back in the autuun holidays 2022, continuing it occasionally and slowly on weekends.
Only about in the winter holidays after, the holidays before I had to present my project. 

The git history was cleaned on Sunday 08 of January (2022) when I created a new repository excluding the unneccessary upper directory structure.

Most of the documntation was done to the end of the holidays, as these are part of the project that is being examined.

## Inspiration and Credit
The idea for this project is not my own. This has ben done before by [David Kiesel] (using unkown methods) and was presented by him in the [C36C3] Congress. 

Credit also goes to [Marudor] and his [bahn.expert] project, it was very helpful for me to understand the inner workings of HAFAS[^hafas].

[David Kiesel]: https://www.dkriesel.com
[C36C3]: https://events.ccc.de/congress/2019/
[Marudor]: https://github.com/marudor
[bahn.expert]: https://bahn.expert
[^hafas]: Hacon Fahrplan Auskunfts System

## Goals
The formulated goal for this project was to query and compare train schedule to realtime data. Sadly, after many experiments I realized this API was designed to resist such data collection attempts. 

I am still trying to find a feasible way (that does not overload the API) to collect such data.

As I could not finish with my targeted goal, I insted selected one more realistic; to display the most used stations in the database.

## Future of the project
I plan to continue my work on this and maybe one day make a good HAFAS library for Rust. This is the primary goal for the future, after which I plan to create a public API that serves better readable responses than HAFAS.

I am always happy to recieve issues and pull requests on my projects, if this project catches on I will surely enjoy maintaining it.

[Eskaan]: https://github.com/eskaan
[GPL3]: https://www.gnu.org/licenses/gpl-3.0.txt
[David Kiesel]: https://www.dkriesel.com
[rustdoc]: doc.rust-lang.org/rustdoc
[mdbook]: rust-lang.github.io/mdBook
[C36C3]: https://events.ccc.de/congress/2019/
[Marudor]: https://github.com/marudor
[bahn.expert]: https://bahn.expert
[^hafas]: Hacon Fahrplan Auskunfts System
