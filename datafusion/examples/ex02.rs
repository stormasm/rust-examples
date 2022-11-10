use datafusion::error::Result;
use datafusion::prelude::*;
use std::env;

/// This example demonstrates executing a simple query against an Arrow data source (CSV) and
/// fetching results
#[tokio::main]
async fn main() -> Result<()> {
    // create local execution context
    let ctx = SessionContext::new();

    let datadir = env::current_dir().unwrap();

    //let testdata1 = String::from("/Users/ma/j/tmp06/rust-examples/datafusion/data/example.csv");
    let testdata = &format!("{}/data/red0.csv", datadir.display());

    // register csv file with the execution context
    ctx.register_csv("example", &testdata, CsvReadOptions::new())
        .await?;
    // execute the query
    let df = ctx
        .sql(
            "Select a from example \
                        where c > 6 \
                        group by a",
        )
        .await?;
    // print the results
    df.show().await?;

    Ok(())
}
