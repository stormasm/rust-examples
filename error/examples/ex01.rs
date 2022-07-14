
#[derive(Debug)]
struct ErrorA;

#[derive(Debug)]
struct ErrorB;

fn my_error_a() -> Result<bool, ErrorA> {
    Err(ErrorA)
}

fn my_error_b() -> Result<bool, ErrorB> {
    Err(ErrorB)
}

fn main() -> Result<bool, ErrorA> {
    my_error_a()?;
    my_error_b()?;
    Ok(true)
}
