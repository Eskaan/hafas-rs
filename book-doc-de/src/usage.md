# Verwendung
Wie der Name der Crate schon sagt, kompiliert `database-cli` in ein Kommandozeilenprogramm[^cli].
Diese Schnittstelle arbeitet mit einer Postgresql-Datenbank zusammen, um HAFAS-Daten zu speichern und anzufordern.

## Liste der Unterbefehle:
Alle Befehle können mit -v oder -vv übergeben werden, um sie ausführlicher zu machen.
Das Argument `--help` kann zu jedem Unterbefehl hinzufügt werden, um Details zu den Argumenten anzuzeigen.
Wenn Sie die CLI nicht kompilieren möchten, finden Sie Argumente im Quellcode.

<details>
<summary>Verantwortlicher Code in `main.rs - zum Erweitern klicken`</summary>

```rust,ignore
{{#include ../../database-cli/src/main.rs:42:84}}
```
</details>

- `data` Dieser Befehl kann nur zusammen mit einem seiner Unterbefehle verwendet werden, er hat keine eigenen Eigenschaften.
   Es sammelt im Allgemeinen Befehle, die verwendet werden, um Suchdaten für Zugfahrten in der Datenbank zu verschieben.

   - `request_raw`: Dieser Befehl fordert rohe Fahrplandaten über alle JIDs im HAFAS-Endpunkt an.
     Unabhängig vom `TO`-Argument gerät der Befehl in Panik, wenn er das letzte Jid erreicht.
     Der übliche letzte Jid liegt bei etwa 1,5 Millionen.

     Dies ist normalerweise der zweite Befehl, der nach `migrate` ausgeführt wird.

     Es ist zu beachten, dass dieser Befehl abhängig von Ihrem Computer und Ihrer Netzwerkverbindung normalerweise ziemlich lange dauert, bis er fertig ist.
     Es wird empfohlen, obwohl es nicht notwendig ist, es mit `--parse` aufzurufen.
    
     Es wird empfohlen, alle anderen optionalen Flags auf einem Standardwert zu belassen, um ein Timeout zu verhindern.

   - `parse`: Dieser Befehl parst die Daten aus der `raw_data`-Tabelle in ein verwendbares Format und fügt sie in die anderen Tabellen ein.
     Es kann auch automatisch aufgerufen werden, indem `--parse` zu den Argumenten von `request_raw` hinzugefügt wird.

     Sie können sich das Datenbankschema und hauptsächlich den [Hafas-Abschnitt] (./hafas.html) ansehen, um weitere Details darüber zu erhalten, wie Daten geparst werden.

   - `parse_heatmap`: Dieser Befehl sollte zu einem bestimmten Zeitpunkt aufgerufen werden, bevor die Funktion `create_heatmap` verwendet wird.
     Es zählt alle aufgezeichneten Zugfahrten in einer eigenen Tabelle für einen schnelleren Zugriff zusammen.
    
     Da naher und lokaler Verkehr das Endbild verschleiern kann, empfehle ich, `-o 'ICE'` als Filter zu setzen.

   - `check`: Dieser Befehl prüft, ob Daten vom HAFAS-Endpunkt von den aktuellen Daten abweichen.
     Diese Prüfung wird nur für einen einzigen Jid durchgeführt. Ein Unterschied könnte auf eine Fahrplanänderung hindeuten.

- `create_heatmap` Erstellt ein horizontales Balkendiagramm der am häufigsten verwendeten Stationen in der Nachschlagetabelle.
   Es kann nach cat_code, cat_out und Suchlimit gefiltert werden. Aus derzeit unbekannten Gründen bringt alles über 11 Balken die Stationsnamen durcheinander.

- `migrate`: Erstellt die gesamte notwendige Infrastruktur auf der entfernten Datenbank. Das CLI selbst arbeitet derzeit nur mit dem lookup_data-Schema, aber dieses `sqlx`-Feature benötigt

Die meisten Befehle führen zu einem Fortschrittsbalken wie diesem:
![Fortschrittsbalken](./images/progress_bar.png)<br>
Die Zählungen am Ende sollen einen ungefähren Hinweis darauf geben, wie lange der Befehl dauern wird.

[^cli]: Englisch [CLI](https://en.wikipedia.org/wiki/Command-line_interface), Command Line Interface
