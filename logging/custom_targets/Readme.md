Here is a simple example of a Rust program that uses the standard log crate along with env\_logger to output messages to **custom log targets**.  
Custom targets allow you to categorize your logs logically (e.g., separating database logs from authentication logs) independently of the module path, making it easier to filter them later.

In the log macros (info\!, warn\!, error\!, etc.), you can specify a custom target using the target: "name" syntax.


### Running and Filtering by Target**

Because you are using env_logger, you can use the RUST_LOG environment variable to control which targets are displayed in your console output.

* **Run all logs (default):**  
```rust  
cargo run
```

* **Show *only* logs from the database target:**  
```rust  
RUST_LOG=database=info cargo run
```
  
* **Show different levels for different targets** (e.g., info for database, error for auth, and info for everything else):  

```rust  
RUST_LOG=info,database=info,auth=error cargo run
```
