use datafusion::error::Result;
use datafusion::prelude::*;
use std::env;

/// This example demonstrates executing a simple query against an Arrow data source (CSV) and
/// fetching results
#[tokio::main]
async fn main() -> Result<()> {
    // create local execution context
    let ctx = SessionContext::new();

    // let testdata = datafusion::test_util::arrow_test_data();
    //let testdata = env::current_dir();

    //let hello = String::from("Hello, world!");

    let testdata = String::from("/Users/ma/j/tmp06/rust-examples/datafusion/data/example.csv");

    // register csv file with the execution context
    ctx.register_csv("example", &testdata, CsvReadOptions::new())
        .await?;

    // execute the query

    let df = ctx.sql("Select * from example").await?;

    /*
    let df = ctx
        .sql(
            "SELECT c1, MIN(c12), MAX(c12) \
        FROM aggregate_test_100 \
        WHERE c11 > 0.1 AND c11 < 0.9 \
        GROUP BY c1",
        )
        .await?;
        */
    // print the results
    df.show().await?;

    Ok(())
}
