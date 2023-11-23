
### Notrace

Given a file with logging statements like *trace!*

Remove those lines of code.

This code does not work when the logging code is embedded in multiple lines.

You must go ahead and manually remove the *trace!* statements that span multiple lines.

That will be version 2 of this code :)

### Usage

```rust
notrace parser.rs > look.txt
```

Then

```rust
rm parser.rs
mv look.txt parser.rs
```

### If you modify notrace.rs

Copy the notrace binary over to your *rustbin* directory anytime
you update notrace.
