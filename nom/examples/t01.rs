//https://iximiuz.com/en/posts/rust-writing-parsers-with-nom/
use nom::{bytes::complete::tag, IResult};

fn foo(s: &str) -> IResult<&str, &str> {
    tag("foo")(s)
}

fn main() {
    assert_eq!(foo("foo bar"), Ok((" bar", "foo")));
    assert!(foo("1234567").is_err());
}
