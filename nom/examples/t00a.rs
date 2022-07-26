use nom::{bytes::complete::is_a, IResult};

fn is_a_error(s: &str) -> IResult<&str, &str> {
    let remote_query: &'static str = "Error";
    is_a(remote_query)(s)
}

fn error_check(s: &str) -> bool {
    let result = is_a_error(s);

    let mybool = match result.is_err() {
        true => true,
        false => false,
    };

    return mybool;
}

fn main() {
    //let result = error_check("Error running remote query: status: InvalidArgument, message: \"Error while planning query: Error during planning:");
    let result = error_check("jack");

    println!("{:?}", result);
}
