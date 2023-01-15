# Projektstruktur
## Allgemeine Struktur
Das Projekt gliedert sich in zwei Hauptteile:
- [hafas-wrap], die Bibliothek[^lib] für den Zugriff auf HAFAS[^hafas].
  
  Es bietet einfache Möglichkeiten, über mehrere Methoden auf das HAFAS zuzugreifen
  und ein generisches Typsystem, das auch benutzerdefinierte Anforderungs- und Antworttypen außerhalb der allgemeinen Verwendung unterstützt.

- [database-cli], das über die Bibliothek abgefragte Daten verwendet und bestimmte Teile zur weiteren Verarbeitung in eine Postgresql-Datenbank einfügt.
  
  Es enthält derzeit auch die Unterbefehle zum ausgeben eines Diagramms mit den am häufigsten verwendeten Stationen in der Datenbank.
  **Wenn Sie nach der Nutzung dieses Projekts suchen, müssen Sie hier nachsehen.**

Der größte Teil des Projekts, wenn auch nicht alles, ist in einem prozedualen Stil geschrieben, da dieses Programm auf einem CLI und einer entsprechenden Bibliothek basiert.
Es gibt mehr objektorientierte Teile in der Bibliothek.

## Weitere Dokumentation
Jeder dieser Projektteile enthält seine eigene `README.md`-Datei, die seine Verwendung weiter erklärt.

Ich empfehle auch, die englische rust-doc Dokumentation für die Dokumentation auf Projektebene zu verwenden, die im Ordner `docs/rustdoc` [`./docs/rustdoc/database_cli/index.html`](/rustdoc/database_cli/index.html) ist.

[hafas-wrap]: ./hafas-wrap.html
[database-cli]: ./Datenbank-Cli.html
[^lib]: Englisch Library, in Rust auch Crate genannt.
[^hafas]: Hacon Fahrplan Auskunfts System
