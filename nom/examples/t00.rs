use nom::{bytes::complete::is_a, IResult};

fn error_check(s: &str) -> IResult<&str, &str> {
    let remote_query: &'static str = "Error";
    is_a(remote_query)(s)
}

fn main() {
    //let result = error_check("Error running remote query: status: InvalidArgument, message: \"Error while planning query: Error during planning:");
    let result = error_check("jack");

    let mybool = match result.is_err() {
        true => true,
        false => false,
    };

    println!("{:?}", mybool);
}
