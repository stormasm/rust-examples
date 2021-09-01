use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

async fn write(s: &[u8]) -> io::Result<()> {
    let mut buffer = File::create("foo.txt").await?;

    buffer.write_all(s).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let myurl = "https://hyper.rs";

    let res = reqwest::get(myurl).await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    //    println!("Body:\n\n{}", body);

    write(&body.into_bytes()).await;

    Ok(())
}
