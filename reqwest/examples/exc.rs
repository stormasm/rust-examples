use tokio::io::{self, AsyncWriteExt};
use tokio::fs::File;

async fn write() -> io::Result<()> {
    let mut buffer = File::create("foo.txt").await?;

    buffer.write_all(b"some bytes").await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let myurl = "https://hyper.rs";

    let res = reqwest::get(myurl).await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    write().await;

    Ok(())
}
