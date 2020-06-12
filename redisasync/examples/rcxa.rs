use redis_client::send;

#[tokio::main]
async fn main() {
    send::set_test().await;
    let res = send::send("set me ohmy").await;
    println!("{:?}",res);
}
