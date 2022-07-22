
## ioxsql

##### ioxsql "select * from h2o_xtemperature"

```rust
"Error running remote query: status: InvalidArgument, message: \"Error while planning query: Error during planning: 'public.iox.h2o_xtemperature' not found\", details: [], metadata: MetadataMap { headers: {\"content-type\": \"application/grpc\", \"date\": \"Wed, 20 Jul 2022 19:08:52 GMT\", \"content-length\": \"0\"} }"
```

##### ioxsql "showx tables"

```rust
"Error running remote query: status: InvalidArgument, message: \"Error while planning query: SQL error: ParserError(\\\"Expected an SQL statement, found: showx\\\")\", details: [], metadata: MetadataMap { headers: {\"content-type\": \"application/grpc\", \"date\": \"Fri, 22 Jul 2022 20:32:33 GMT\", \"content-length\": \"0\"} }"
```

* [go back to Readme](./Readme.md)