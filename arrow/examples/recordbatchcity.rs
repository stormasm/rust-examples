use arrow::array::{ArrayRef, BooleanArray, Int32Array, StringArray, StructArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

#[cfg(feature = "prettyprint")]
use arrow::util::pretty::print_batches;

fn main() -> arrow::error::Result<()> {
    //let id0 = Int32Array::from(vec![1, 2, 3, 4, 5]);
    //let id1 = Int32Array::from(vec![6, 7, 8, 9, 10]);

    let city = Arc::new(StringArray::from(vec![
        Some("santafe"),
        Some("socorro"),
        Some("lascruces"),
        Some("abq"),
    ]));

    let coop = Arc::new(BooleanArray::from(vec![true, false, false, true]));
    let pop = Arc::new(Int32Array::from(vec![121, 28, 125, 631]));

    let struct_array1 = StructArray::from(vec![
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

    let struct_array2 = StructArray::from(vec![
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

    // println!("{:?}", struct_array);

    let batch = RecordBatch::try_new(
        Arc::new(Schema::new(vec![
            Field::new(
                "cityg1",
                DataType::Struct(vec![
                    Field::new("city", DataType::Utf8, false),
                    Field::new("coop", DataType::Boolean, false),
                    Field::new("pop", DataType::Int32, false),
                ]),
                false,
            ),
            Field::new(
                "cityg2",
                DataType::Struct(vec![
                    Field::new("city", DataType::Utf8, false),
                    Field::new("coop", DataType::Boolean, false),
                    Field::new("pop", DataType::Int32, false),
                ]),
                false,
            ),
        ])),
        vec![Arc::new(struct_array1), Arc::new(struct_array2)],
    )?;

    print_batches(&[batch.clone()]).unwrap();

    Ok(())
}
