/// A nested array type where each child (called *field*)
/// is represented by a separate array

/// # Example: Create an array from a vector of fields
use arrow::array::{Array, ArrayRef, BooleanArray, Int32Array, StructArray};
use arrow::datatypes::{DataType, Field};
use std::sync::Arc;

fn main() -> arrow::error::Result<()> {
    let boolean = Arc::new(BooleanArray::from(vec![false, false, true, true]));
    let int = Arc::new(Int32Array::from(vec![42, 28, 19, 31]));

    let struct_array = StructArray::from(vec![
        (
            Field::new("b", DataType::Boolean, false),
            boolean.clone() as ArrayRef,
        ),
        (
            Field::new("c", DataType::Int32, false),
            int.clone() as ArrayRef,
        ),
    ]);
    assert_eq!(struct_array.column(0).as_ref(), boolean.as_ref());
    assert_eq!(struct_array.column(1).as_ref(), int.as_ref());
    assert_eq!(4, struct_array.len());
    assert_eq!(0, struct_array.null_count());
    assert_eq!(0, struct_array.offset());
    Ok(())
}
