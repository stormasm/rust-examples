
### Parse this string which is an iox error

```rust
"Error running remote query: status: InvalidArgument, message: \"Error while planning query: Error during planning: 'public.iox.h2o_xtemperature' not found\", details: [], metadata: MetadataMap { headers: {\"content-type\": \"application/grpc\", \"date\": \"Wed, 20 Jul 2022 19:08:52 GMT\", \"content-length\": \"0\"} }"
```

### Nom references

* [nom](https://github.com/Geal/nom)
* [nom api](https://docs.rs/nom/latest/nom/)
* [nom crates.io](https://crates.io/crates/nom)
* [nom doc: List of parsers and combinators](https://github.com/Geal/nom/blob/main/doc/choosing_a_combinator.md)

### Reference Articles

* [Rust - Writing Parsers With nom Parser Combinator Framework](https://iximiuz.com/en/posts/rust-writing-parsers-with-nom/)

### Github Projects using Nom

* [pq](https://github.com/iximiuz/pq)
* [config-rs](https://github.com/mehcode/config-rs)