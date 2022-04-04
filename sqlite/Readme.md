
Because the csv vtab functionality is new it is not currently built into the sqlite3 default shell as well as the sqlite browser explorer therefore we have created some simple tools to dump the database out to the terminal.

To create a sqlite database from a csv file and then to view the entire sqlite database.

```rust
cre dbcreate people8.db ./csv/people1.csv
cre dbview people8.db
```

* Step 1 is to create the database using
[csvs-to-sqlite](https://github.com/simonw/csvs-to-sqlite)

```rust
csvs-to-sqlite ./csv/moravec.csv mydb1.db
```

* Step 2 is to read back the database

```rust
cargo run --example ex01
```

Note: You have to put the entry column data in the csv file in a String, you can not just have it be a bunch of bare characters without the string delineation. And it looks Strings are best delineated with double quotes and not single quotes.

#### sqlite3 command line tool

To visualize your sqlite db you can simply run the command line tool sqlite3.

For more details go [here](https://sqlite.org/cli.html).

For specific commands needed in this repo checkout our
[sqlite3 cheat sheet](./sqlite3.md)

### References

* [The Virtual Table Mechanism Of SQLite](https://www.sqlite.org/vtab.html)
* [The CSV Virtual Table](https://www.sqlite.org/csv.html)
* [JSON Functions And Operators](https://www.sqlite.org/json1.html)
