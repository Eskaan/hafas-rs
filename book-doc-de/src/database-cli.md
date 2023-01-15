# Die Datenbank-CLI-Toolchain
Dies ist derzeit die einzige Implementierung der oben genannten Bibliothek.

Die aktuelle Verwendung besteht darin, eine Datenbank mit Fahrten, Zugtypen, Standorten und Betreibern zu erstellen.
Das genaue Layout findet man unter `database_layout.png` im Stammverzeichnis des Projekts.
Man kann auch ein einfaches Diagramm aus Nutzungsstatistiken erstellen.

Obwohl das Programm keine Live-Datenbank zum Kompilieren benötigt, benötigt es eine Postgres-Datenbank, um zu funktionieren.
Die Datenbank muss auf localhost liegen, mit dem Namen `db-statistics` und dem Benutzer `postgres` (passwortlos)
auf alle Tabellen zugreifen zu können, die im Datenbanklayout angegeben sind.
Um die Datenbank für die Verwendung vorzubereiten, führen Sie `database-climigrate` aus. Dadurch werden alle erforderlichen Schemas, Tabellen, Typen und Funktionen erstellt.

Die genaue Verwendung finden Sie in der Rustdoc-Dokumentation der Methode [`main`] oder im Abschnitt über die Nutzung im Buch.
Diese Crate ist größtenteils in einem prozeduraleren (aber auch teilweise objektorientierten) Programmierstil geschrieben, wie es für eine CLI-Schnittstelle zu einer Bibliothek üblich ist.
Daraus resultieren viele Module, die meist einzelne Funktionen anstelle ganzer Objekte enthalten.

Für die Modulebene empfehle ich wieder die Rust-Doc-Dokumentation für diese Crate, die [hier](./databasecli/index.html) verfügbar ist.

Besser strukturierte Informationen zu den CLI-Befehlen finden Sie in der [Nutzungssektion](./usage.html) dieses Buchs.

## Main
Dies ist das Wurzelmodul[^root] der Crate, hier liegt die Funktion, die beim Programmstart aufgerufen wird. Dieses Modul analysiert die CLI-Argumente und ruft die entsprechenden Funktionen auf.

##request_raw_jids
Dieses Modul ist für den Unterbefehl `data request_raw` verantwortlich. Es fordert Rohdaten von HAFAS an und schreibt sie in die Tabelle `raw_data`.

## parse_raw_jids
Dieses Modul wird vom Unterbefehl `data parse` aufgerufen und analysiert Daten aus den `raw_data` und fügt sie in die anderen Tabellen `trips`, `locations`, `operators` und `train_types` ein.

## count_location_trips
Dieses Modul ist eine Erweiterung des Unterbefehls `data parse`, der mit `dara parse_heatmap` aufgerufen wird.

## vergleiche_rohdaten
Dieses Modul wird vom Unterbefehl "Datenprüfung" aufgerufen. Es verwendet das Modul `request_raw_jids`, um ein einzelnes JID anzufordern und es mit der Tabelle `raw_data` zu vergleichen.

## create_heatmap_diagram
Dieses Modul enthält neben dem Herumspielen mit Datenbankdaten den einzigen praktischen Nutzen, den ich bisher gefunden habe. Es wird vom Unterbefehl `create_heatmap` aufgerufen und erstellt ein horizontales Diagramm, das die am häufigsten verwendeten Stationen in der Datenbank anzeigt.

[^root]: Englisch Root Module, in Java die Klasse mit main Funktion.
