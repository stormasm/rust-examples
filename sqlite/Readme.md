
* Step 1 is to create the database using
[csvs-to-sqlite](https://github.com/simonw/csvs-to-sqlite)

```rust
csvs-to-sqlite ./csv/moravec.csv mydb1.db
```

* Step 2 is to read back the database

```rust
cargo run --example ex01
```
