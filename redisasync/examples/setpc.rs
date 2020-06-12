// This uses paired_connect instead of connect
// which is shown in the example set.rs

use redis_async::{client, resp_array};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6379);
    assert_eq!("127.0.0.1:6379".parse(), Ok(addr));

    let connection = client::paired_connect(&addr)
        .await
        .expect("Cannot open connection");

    let res_f = connection.send(resp_array!["PING", "Rick"]);
    connection.send_and_forget(resp_array!["SET", "x", "1234"]);
    let wait_f = connection.send(resp_array!["GET", "x"]);

    let result_1: String = res_f.await.expect("Cannot read result of first thing");
    let result_2: String = wait_f.await.expect("Cannot read result of second thing");

    assert_eq!(result_1, "Rick");
    assert_eq!(result_2, "1234");

    // Now lets work on taking a string from the terminal
    // and munging it into vec which is the input to the resp_array

    let myiter = "set y 5678".split_whitespace();

    let myvec = myiter
        .map(|item| {
            item
        })
        .collect::<Vec<_>>();

    assert_eq!(myvec,vec!["set", "y", "5678"]);

    let command = resp_array![].append(myvec);

//  connection.send_and_forget(resp_array!["SET", "y", "5678"]);
    connection.send_and_forget(command);

    let wait_g = connection.send(resp_array!["GET", "y"]);
    let result_3: String = wait_g.await.expect("Cannot read result of third thing");
    assert_eq!(result_3, "5678");
}
