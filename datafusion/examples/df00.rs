use datafusion::error::Result;
use datafusion::prelude::*;
#[tokio::main]
async fn main() -> Result<()> {
    let ctx = SessionContext::new();
    let df = ctx.read_csv("data/red0.csv", CsvReadOptions::new()).await?;
    df.collect().await?;
    // print the results
    // df.show().await?;
    Ok(())
}
