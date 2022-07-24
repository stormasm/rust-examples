use nom::{bytes::complete::take_until, IResult};

#[derive(Debug)]
enum NuIoxErrorType {
    SQLTABLE,
    SQLSHOW,
}

#[derive(Debug)]
struct NuIoxError {
    start: String,
    error_type: NuIoxErrorType,
    header: String,
    status: String,
    message: String,
}

impl NuIoxError {
    fn build(&mut self, mut data: &str) -> Self {
        let details = remove_details(data).unwrap().1;
        let (message0, remainder) = get_message(details).unwrap();
        let (status0, header0) = get_header(&remainder).unwrap();

        let header1 = remove_colon_from_string(&header0.to_string());
        println!("{:?}", header1.trim());

        println!("{:?}", &status0);

        let message1 = remove_slash_from_string(&message0.to_string());
        println!("{:?}", message1.trim());

        Self {
            start: data.to_string(),
            error_type: NuIoxErrorType::SQLTABLE,
            header: header1,
            status: status0.to_string(),
            message: message1,
        }
    }
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
}
