use nom::{bytes::complete::take_until, IResult};

enum NuIoxErrorType {
    SQLTABLE,
    SQLSHOW,
}

struct NuIoxError {
    start: String,
    error_type: NuIoxErrorType,
    header: String,
    status: String,
    message: String,
}

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

fn remove_slash_from_string(s: &String) -> String {
    s.replace(&['(', ')', ',', '\"', ';', '\''][..], "")
}

fn remove_colon_from_string(s: &String) -> String {
    s.replace(&[':'][..], "")
}

fn main() {
    let data: &'static str = "Error running remote query: status: InvalidArgument, message: \"Error while planning query: Error during planning: 'public.iox.h2o_xtemperature' not found\", details: [], metadata: MetadataMap { headers: {\"content-type\": \"application/grpc\", \"date\": \"Wed, 20 Jul 2022 19:08:52 GMT\", \"content-length\": \"0\"} }";

    let details = remove_details(data).unwrap().1;
    let (message, remainder) = get_message(details).unwrap();
    let (status, header) = get_header(&remainder).unwrap();

    let header1 = remove_colon_from_string(&header.to_string());
    println!("{:?}", header1.trim());

    println!("{:?}", &status);

    let message1 = remove_slash_from_string(&message.to_string());
    println!("{:?}", message1.trim());
}
