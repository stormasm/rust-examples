use nom::{bytes::complete::tag, IResult};

fn part00(s: &str) -> IResult<&str, &str> {
    let remote_query: &'static str = "Error running remote query: ";
    tag(remote_query)(s)
}

fn main() {
    let result = part00("Error running remote query: status: InvalidArgument, message: \"Error while planning query: Error during planning:");
    println!("{:?}", result);
}
