# HAFAS und Datenbanklayout
## Datenbanklayout
![Layout](./images/db_layout.png)
<details>
<summary>DDL für Datenbanklayout - zum Erweitern klicken</summary>

```SQL
{{#include ../../database-cli/migrations/2022_12_21_1_init.sql}}
```
</details>

Die gesamte Datenbank ist derzeit im lookup_data-Schema enthalten.

Das allgemeine Schema ist oben dargestellt, mit einigen benutzerdefinierten zusammengesetzten Typen und Funktionen, die hier erklärt werden:
- `operation_dates` ist ein Typ, der ein Array von `dates`, ein `from_loc`-Eva mit entsprechendem `to_loc`-Eva und einen `Info`-Text enthält.
- `scheduled_stop` ist ein Typ, der einen Halt einer Fahrt an einer Station beschreibt. Es enthält die `eva`, die `scheduled_arrival`-Zeit und die entsprechende `scheduled_departure`-Zeit.
- `from_evas` ist eine Funktion, die ein Array von `scheduled_stop`-Elementen verwendet und nur das `eva` als int-Array daraus extrahiert.
