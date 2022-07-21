/*
Leave everything in the string except a certain part of the string
*/

use nom::{bytes::complete::take_until, IResult};

fn remove00(s: &str) -> IResult<&str, &str> {
    // let remote_query: &'static str = "metadata: ";
    take_until(" metadata: ")(s)
}

fn main() {
    let data: &'static str = "details: [], metadata: MetadataMap { headers: {\"content-type\": \"application/grpc\", \"date\": \"Wed, 20 Jul 2022 19:08:52 GMT\", \"content-length\": \"0\"} }";
    let result = remove00(data);
    println!("{:?}", result);
}
