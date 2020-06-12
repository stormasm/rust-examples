use redis_client::{send};

#[tokio::main]
async fn main() {
    send::rick().await;
}
