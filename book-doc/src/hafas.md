# HAFAS and Database layout
## Database Layout
![Layout](./images/db_layout.png)
<details>
<summary>DDL for Database Layout - click to expand</summary>

```sql
{{#include ../../database-cli/migrations/2022_12_21_1_init.sql}}
```
</details>

The whole database is currently contained to the lookup_data scheme.

The common scheme is shown above, with a few custom composite types and functions explained here:
- `operation_dates` is a type that contains an array of `dates`, a `from_loc` eva with corresponding `to_loc` eva and a `info` text.
- `scheduled_stop` is a type describing a stop of a trip at a station. It contains the `eva`, the `scheduled_arrival` time and corresponding `scheduled_departure` time.
- `from_evas` is a function taking an array of `scheduled_stop` elements, extracting only the `eva` as int array from it.
