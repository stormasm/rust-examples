// https://docs.rs/redis-async/0.6.3/redis_async/resp/index.html
// used to convert a string that you would type into redis client
// into something that this would understand as a resp_array
// https://github.com/benashford/redis-async-rs/blob/master/src/resp.rs

fn main() {

    let myiter = "set nm raton".split_whitespace();

    let vec = myiter
        .map(|item| {
            item
        })
        .collect::<Vec<_>>();

    assert_eq!(vec,["set", "nm", "raton"])
}
