use redis_client::send;

// ok

#[tokio::main]
async fn main() {
    send::set_test().await;
}
