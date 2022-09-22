use arrow::datatypes::{DataType, Field};
use arrow::util::data_gen::create_random_array;

fn main() -> arrow::error::Result<()> {
    let _x1 = chk1();
    let _x2 = chk2();
    Ok(())
}

fn chk2() -> arrow::error::Result<()> {
    let size = 1;
    let struct_fields = vec![
        Field::new("b", DataType::Boolean, true),
        Field::new(
            "c",
            DataType::LargeList(Box::new(Field::new(
                "item",
                DataType::List(Box::new(Field::new(
                    "item",
                    DataType::FixedSizeBinary(6),
                    true,
                ))),
                false,
            ))),
            true,
        ),
        Field::new(
            "d",
            DataType::Struct(vec![
                Field::new("d_x", DataType::Int32, true),
                Field::new("d_y", DataType::Float32, false),
                Field::new("d_z", DataType::Binary, true),
            ]),
            true,
        ),
    ];
    let field = Field::new("struct", DataType::Struct(struct_fields), true);
    let array = create_random_array(&field, size, 0.2, 0.5).unwrap();
    println!("{:?}", array);
    Ok(())
}

fn chk1() -> arrow::error::Result<()> {
    let size = 2;
    let field = Field::new("id0", DataType::Int32, false);

    let _array = create_random_array(&field, size, 0.2, 0.5).unwrap();
    //println!("{:?}", array);
    Ok(())
}
