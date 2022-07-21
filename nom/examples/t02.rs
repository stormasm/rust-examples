use nom::{bytes::complete::tag, IResult};

fn foo(s: &str) -> IResult<&str, &str> {
    tag("foo")(s)
}

fn main() {
    let result = foo("foo bar");
    println!("{:?}", result);
}
