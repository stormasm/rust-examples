
### Notrace

Given a file with logging statements like *trace!*

This code will remove those lines of code.

This code does not work when the logging code spans multiple lines.

You must go ahead and manually remove the *trace!* statements that span multiple lines.

That will be version 2 of this code :)

### Building this code

```rust
cargo build
```

Then copy the notrace binary over to your *rustbin* directory anytime
you update notrace.

### Usage

```rust
notrace parser.rs > look.txt
```

Then

```rust
rm parser.rs
mv look.txt parser.rs
```
