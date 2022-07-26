use nom::{bytes::complete::is_a, IResult};

pub fn is_a_error(s: &str) -> IResult<&str, &str> {
    let remote_query: &'static str = "Error";
    is_a(remote_query)(s)
}

// This returns true if there is not the word Error in the string,
// meaning that an error was thrown by nom because it can not find the Error string
// This returns false if the string has the word Error in it
pub fn error_check(s: &str) -> bool {
    let result = is_a_error(s);
    println!("error_check result 2 = {:?}", result);

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
