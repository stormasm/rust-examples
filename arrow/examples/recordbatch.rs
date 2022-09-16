use arrow::array::Int32Array;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

#[cfg(feature = "prettyprint")]
use arrow::util::pretty::print_batches;

fn main() -> arrow::error::Result<()> {
    let id0 = Int32Array::from(vec![1, 2, 3, 4, 5]);
    let id1 = Int32Array::from(vec![6, 7, 8, 9, 10]);

    let batch = RecordBatch::try_new(
        Arc::new(Schema::new(vec![
            Field::new("id0", DataType::Int32, false),
            Field::new("id1", DataType::Int32, false),
        ])),
        vec![Arc::new(id0), Arc::new(id1)],
    )?;

    let int32array = batch
        .column(1)
        .as_any()
        .downcast_ref::<Int32Array>()
        .expect("Failed to downcast");

    print_batches(&[batch.clone()]).unwrap();
    println!("{:?}", int32array);
    Ok(())
}
