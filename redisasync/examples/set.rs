// This uses connect instead of paired_connect
// which is shown in the example setpc.rs

use futures::{sink::SinkExt};
use redis_async::{client, resp_array};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6379);
    assert_eq!("127.0.0.1:6379".parse(), Ok(addr));

    let mut connection = client::connect(&addr)
        .await
        .expect("Cannot connect to Redis");
    connection
        .send(resp_array!["SET", "nm", "raton 1"])
        .await
        .expect("Cannot send SET command");
}
