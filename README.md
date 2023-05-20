# HAFAS-RS
### Status
Currently this library only implements a very small part of HAFAS and is very much not production ready. The library is built in such a way, that you can make requests using your own data structs if you want, but I'd recommend making a pull request and adding it to the library. **Any help is welcome.**

### Development of a program written in Rust to query "Deutsche Bahn"-data

## Background
The [Deutsche Bahn AG] (abbreviated DB) is the German national railway company. As DB is
one of the largest transport companies of the world and the largest railway operator in
Europe (according to [this](https://web.archive.org/web/20170927052239/http://www.railway-technology.com/features/featureengines-of-trade-the-ten-biggest-rail-companies-by-revenue-4943955/featureengines-of-trade-the-ten-biggest-rail-companies-by-revenue-4943955-1.html) source), a complex network of time schedules, trains and routes in Germany and other
European countries has to be organized. To facilitate the online journey planner for
customers the Software [HAFAS] (HaCon Fahrplan-Auskunfts-System) of the company HaCon
(Hannover Consulting, belonging to Siemens) is used. 
[Rust] is a general-purpose programming language with outstanding performance and safety.
It doesn’t require the use of a garbage collector or reference counting to ensure memory
safety. Rust can be used for systems programming whilst also offering high-level features.
For example, programs can be created with Rust to draw data from different software e.g.
HAFAS.

## Documentation
The recommended way to access the documentation is through the generated HTML version in the docs folder.
There is both `rustdoc` as `mdbook` documentation on this project, found in the `docs` folder.
- [Rustdoc](https://eskaan.github.io/hafas-rs/rustdoc/database_cli/index.html)
- [[en] Book](https://eskaan.github.io/hafas-rs/book/)
- [[de] Book](https://eskaan.github.io/hafas-rs/book-de/)

## About the project
The purpous of this project named HAFAS-RS, done for a high school presentation (GFS) by Adrian Struwe 
([Eskaan]), was to develop a program to collect and list targeted data on trips, train types, locations and operators of the Deutsche Bahn.

It is currently licenced under the [GPL3] licence, allowing open distribution and modification of the code.
The project is fully documented, both in source code (per [rustdoc]) as in this [mdbook].

[Deutsche Bahn AG]: https://www.bahn.de/
[HAFAS]: https://de.wikipedia.org/wiki/HAFAS
[Rust]: https://www.rust-lang.org/
[Eskaan]: https://github.com/eskaan
[GPL3]: https://www.gnu.org/licenses/gpl-3.0.txt
[rustdoc]: doc.rust-lang.org/rustdoc
[mdbook]: rust-lang.github.io/mdBook


## Inspiration and Credit
The idea for this project is not my own. This has been done before by [David Kriesel] (using unkown methods) and was presented in the [C36C3] Congress. 

Credit also goes to [Marudor] and his [bahn.expert] project, it was very helpful for me to understand the inner workings of HAFAS[^hafas].

[David Kriesel]: https://www.dkriesel.com
[C36C3]: https://events.ccc.de/congress/2019/
[Marudor]: https://github.com/marudor
[bahn.expert]: https://bahn.expert
[^hafas]: Hacon Fahrplan Auskunfts System

## Goals
The primary goal of the project was to extract data, e.g. the most used stations, and display them in a database.

Another goal was to query and compare train schedules to realtime data. 
Sadly, after many experiments I realized this API was designed to resist such data collection attempts. 

In the future, I want try to find a feasible way (that does not overload the API) to collect such data.


## Future of the project
I plan to continue my work on this project and maybe one day develop a good HAFAS library for Rust. This is the primary goal for the future, after which I plan to create a public API that serves better readable responses than HAFAS.

I am always happy to receive issues and pull requests on my projects, if this project catches on I will surely enjoy maintaining it.

[Eskaan]: https://github.com/eskaan
[GPL3]: https://www.gnu.org/licenses/gpl-3.0.txt
[David Kiesel]: https://www.dkriesel.com
[rustdoc]: doc.rust-lang.org/rustdoc
[mdbook]: rust-lang.github.io/mdBook
[C36C3]: https://events.ccc.de/congress/2019/
[Marudor]: https://github.com/marudor
[bahn.expert]: https://bahn.expert
[^hafas]: Hacon Fahrplan Auskunfts System
