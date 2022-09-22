use arrow::datatypes::{DataType, Field};
use arrow::util::data_gen::create_random_array;

fn main() -> arrow::error::Result<()> {
    let size = 2;
    let field = Field::new("id0", DataType::Int32, false);

    let array = create_random_array(&field, size, 0.2, 0.5).unwrap();
    println!("{:?}", array);
    Ok(())
}
