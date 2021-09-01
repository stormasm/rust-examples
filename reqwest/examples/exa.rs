#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let myurl = "https://hyper.rs";

    let res = reqwest::get(myurl).await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}
