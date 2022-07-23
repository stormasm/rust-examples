/*
Leave everything in the string except a certain part of the string
*/

use nom::{bytes::complete::take_until, IResult};

fn remove_details(s: &str) -> IResult<&str, &str> {
    let details: &'static str = ", details: ";
    take_until(details)(s)
}

fn get_message(s: &str) -> IResult<&str, &str> {
    let msg: &'static str = ", message: ";
    take_until(msg)(s)
}

fn get_header(s: &str) -> IResult<&str, &str> {
    let header: &'static str = "status: ";
    take_until(header)(s)
}

fn remove_char_from_string(s: &String) -> String {
    s.replace(&['(', ')', ',', '\"', ';', '\''][..], "")
}

fn main() {
    // let data: &'static str = "during planning: 'public.iox.h2o_xtemperature' not found\", details: [], metadata: MetadataMap { headers: {\"content-type\": \"application/grpc\", \"date\": \"Wed, 20 Jul 2022 19:08:52 GMT\", \"content-length\": \"0\"} }";
    let data: &'static str = "Error running remote query: status: InvalidArgument, message: \"Error while planning query: Error during planning: 'public.iox.h2o_xtemperature' not found\", details: [], metadata: MetadataMap { headers: {\"content-type\": \"application/grpc\", \"date\": \"Wed, 20 Jul 2022 19:08:52 GMT\", \"content-length\": \"0\"} }";
    let details = remove_details(data).unwrap().1;
    let (message, remainder) = get_message(details).unwrap();
    println!("{:?}\n{:?}", &remainder, &message);
    let (status, header) = get_header(&remainder).unwrap();
    println!("{:?}", &header);
    println!("{:?}", &status);
    let msg1 = remove_char_from_string(&message.to_string());
    println!("{:?}", msg1.trim());
}
