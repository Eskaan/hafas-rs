# The database-cli toolchain
This is the (currently) only implementation of the aforementioned library.

{{#include ../../database-cli/README.md:5:14}}

This crate is mostly written in a more procedual (althrough object-oriented) programming style, as common for a cli interface to a library.
This results in many modules which mostly contain single functions instead of entire objects. 

For this module-level I highly recommend the rust-doc documentation for this crate available [here](./databasecli/index.html)

For better structured information on the cli commands, see the [usage section](./usage.html) of this book.

## Main
This is the root module of the crate, here lies the function that is called on program start. This module parses the cli arguments and calls the corresponding functions.

## request_raw_jids
This module is responsible for the `data request_raw` subcommand. It requests raw data from HAFAS and writes it to the `raw_data` table.

## parse_raw_jids
This module gets called from the `data parse` subcommand and parses data from the `raw_data` and inserts it into the other tables `trips`,`locations`,`operators` and `train_types`.

## count_location_trips
This module is a extension to the `data parse` subcommand, called by using `dara parse_heatmap`. It 

## compare_raw_data
This module is called from the `data check` subcommand. It uses the request_raw_jids module to request a single jid and compare it to the `raw_data` table.

## create_heatmap_diagram
This module contains the only practical use besides playing with database data I have fount yet. It is called by the `create_heatmap` subcommand and creates a horizontal chart diagram displaying the most used stations in the database.