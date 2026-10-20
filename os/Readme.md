
How to see my target triple

```rust
rustc -vV | sed -n 's|host: ||p'
```

which turns out to be

```rust
aarch64-apple-darwin
```

This is different from my target_os

### References

- https://users.rust-lang.org/t/get-target-os-as-str-or-string/76282
- https://doc.rust-lang.org/reference/conditional-compilation.html
