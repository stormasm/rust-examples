fn get_host_and_port(path: &str) -> (&str, usize) {
    let mut index = path.find(":").unwrap();
    let (host, _tmp) = path.split_at(index);
    index = index + 1;
    let (_tmp, port) = path.split_at(index);
    let port: usize = port.parse().unwrap();
    (host, port)
}

fn main() {
    let path = "127.0.0.1:50060";
    let (host, port) = get_host_and_port(path);
    println!("host = {} port = {}", host, port);
}
