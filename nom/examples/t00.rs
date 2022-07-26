use nom::{bytes::complete::is_a, IResult};

fn is_a_error(s: &str) -> IResult<&str, &str> {
    let remote_query: &'static str = "Error";
    is_a(remote_query)(s)
}

fn main() {
    let result = is_a_error("Error running remote query: status: InvalidArgument, message: \"Error while planning query: Error during planning:");
    //let result = is_a_error("jack");

    let mybool = match result.is_err() {
        true => true,
        false => false,
    };

    println!("{:?}", mybool);
}
