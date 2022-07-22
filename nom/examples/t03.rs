/*
Leave everything in the string except a certain part of the string
*/

use nom::{bytes::complete::take_until, IResult};

fn remove00(s: &str) -> IResult<&str, &str> {
    let delete: &'static str = ", details: ";
    take_until(delete)(s)
}

fn main() {
    // let data: &'static str = "during planning: 'public.iox.h2o_xtemperature' not found\", details: [], metadata: MetadataMap { headers: {\"content-type\": \"application/grpc\", \"date\": \"Wed, 20 Jul 2022 19:08:52 GMT\", \"content-length\": \"0\"} }";
    let data: &'static str = "Error running remote query: status: InvalidArgument, message: \"Error while planning query: Error during planning: 'public.iox.h2o_xtemperature' not found\", details: [], metadata: MetadataMap { headers: {\"content-type\": \"application/grpc\", \"date\": \"Wed, 20 Jul 2022 19:08:52 GMT\", \"content-length\": \"0\"} }";
    let result = remove00(data).unwrap().1;
    println!("{:?}", result);
}
