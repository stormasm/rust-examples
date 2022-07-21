use nom::{bytes::complete::tag, IResult};

fn foo(s: &str) -> IResult<&str, &str> {
    tag("Error running remote query:")(s)
}

fn main() {
    let result = foo("Error running remote query:");
    println!("{:?}", result);
}
