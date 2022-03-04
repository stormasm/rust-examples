
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
