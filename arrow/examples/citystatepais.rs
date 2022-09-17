use arrow::array::{BooleanArray, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

#[cfg(feature = "prettyprint")]
use arrow::util::pretty::print_batches;

fn main() -> arrow::error::Result<()> {
    let name = Arc::new(StringArray::from(vec![
        Some("santafe"),
        Some("socorro"),
        Some("abq"),
    ]));
    let coop = Arc::new(BooleanArray::from(vec![true, false, true]));
    let pop = Arc::new(Int32Array::from(vec![121, 32, 621]));

    let batch = RecordBatch::try_new(
        Arc::new(Schema::new(vec![
            Field::new("name", DataType::Utf8, false),
            Field::new("coop", DataType::Boolean, false),
            Field::new("pop", DataType::Int32, false),
        ])),
        vec![Arc::new(name), Arc::new(coop), Arc::new(pop)],
    )?;

    print_batches(&[batch.clone()]).unwrap();
    Ok(())
}
