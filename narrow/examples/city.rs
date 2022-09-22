/// A nested array type where each child (called *field*)
/// is represented by a separate array

/// # Example: Create an array from a vector of fields
use arrow::array::{ArrayRef, BooleanArray, Int32Array, StringArray, StructArray};
use arrow::datatypes::{DataType, Field};
use std::sync::Arc;

fn main() -> arrow::error::Result<()> {
    let city = Arc::new(StringArray::from(vec![
        Some("santafe"),
        Some("socorro"),
        Some("lascruces"),
        Some("abq"),
    ]));
    let coop = Arc::new(BooleanArray::from(vec![true, false, false, true]));
    let pop = Arc::new(Int32Array::from(vec![121, 28, 125, 631]));

    let struct_array = StructArray::from(vec![
        (
            Field::new("city", DataType::Utf8, false),
            city.clone() as ArrayRef,
        ),
        (
            Field::new("coop", DataType::Boolean, false),
            coop.clone() as ArrayRef,
        ),
        (
            Field::new("pop", DataType::Int32, false),
            pop.clone() as ArrayRef,
        ),
    ]);
    println!("{:?}", struct_array);
    Ok(())
}
