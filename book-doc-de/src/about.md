# HAFAS-RS
### Entwicklung eines in Rust geschriebenen Programms zur Abfrage von "Deutsche Bahn"-Daten

## Hintergrund
Die [Deutsche Bahn AG] (kurz DB oder auch die Bahn) ist das staatliche Eisenbahnunternehmen Deutschlands. Die DB ist
eines der größten Transportunternehmen der Welt und der größte Eisenbahnbetreiber in
Europa (laut [dieser](https://web.archive.org/web/20170927052239/http://www.railway-technology.com/features/featureengines-of-trade-the-ten-biggest-rail- Companies-by-Revenue-4943955/featureengines-of-trade-the-tent-biggest-rail-companys-by-Revenue-4943955-1.html) Quelle), wobei ein komplexes Netzwerk aus Fahrplänen, Zügen und Strecken in Deutschland und andere
Europäische Länder müssen organisiert werden müssen. Zur Erleichterung der Online-Fahrplanauskunft 
wird die Software [HAFAS] (HaCon Fahrplan-Auskunfts-System) der Firma HaCon
(Hannover Consulting, gehörend zu Siemens) verwendet.
[Rust] ist eine System-Programmiersprache mit hervorragender Leistung und Sicherheit.
Es ist nicht erforderlich, einen Müllsammler[^gc] oder Referenzzählung[^refc] zu verwenden, um die Speichersicherheit[^memsafety] sicherzustellen.
Sicherheit. Rust kann für die Systemprogrammierung verwendet werden und bietet gleichzeitig High-Level-Funktionen.
Beispielsweise können mit Rust Programme erstellt werden, um Daten aus unterschiedlichsten Programmierschnittstellen[^apis] zu ziehen, z.B.
HAFAS.

## Dokumentation
Der empfohlene Zugriff auf die Dokumentation erfolgt über die generierte HTML-Version im Ordner `docs`.
Es gibt sowohl eine `rustdoc`- als auch eine `mdbook`-Dokumentation zu diesem Projekt, die sich im Ordner `docs` befindet.
- [Rustdoc](https://eskaan.github.io/hafas-rs/rustdoc/database_cli/index.html)
- [[en] Buch](https://eskaan.github.io/hafas-rs/book/)
- [[de] Buch](https://eskaan.github.io/hafas-rs/book-de/)

## Über das Projekt
Das Zeil dieses Projekts mit dem Namen HAFAS-RS, für eine Highschool-Präsentation (GFS) von Adrian Struwe
([Eskaan]) erstellt, war ein Programm zu entwickeln, um gezielt Daten zu Fahrten, Zugtypen, Standorten und Betreibern der Deutschen Bahn zu sammeln und aufzulisten.

Es ist derzeit unter der [GPL3]-Lizenz lizenziert, die eine offene Verteilung und Änderung des Codes erlaubt.
Das Projekt ist vollständig dokumentiert, sowohl im Quellcode (per [rustdoc]) als auch in diesem [mdbook].

[Deutsche Bahn AG]: https://www.bahn.de/
[HAFAS]: https://de.wikipedia.org/wiki/HAFAS
[Rust]: https://www.rust-lang.org/
[Eskaan]: https://github.com/eskaan
[GPL3]: https://www.gnu.org/licenses/gpl-3.0.txt
[rustdoc]: doc.rust-lang.org/rustdoc
[mdbook]: rust-lang.github.io/mdBook
[^gc]: Englisch [Garbage Collector](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science))
[^refc]: Englisch [Reference Counting](https://en.wikipedia.org/wiki/Reference_counting)
[^memsafety]: Englisch [Memory Safety](https://en.wikipedia.org/wiki/Memory_safety)
[^apis]: Englisch [API](https://en.wikipedia.org/wiki/API), Application Programming Interface
