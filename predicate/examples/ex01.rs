use predicates::prelude::*;

fn main() {
    let predicate_fn = predicate::str::contains("Two Three");
    assert_eq!(true, predicate_fn.eval("One Two Three"));
    assert_eq!(false, predicate_fn.eval("Four Five Six"));

    let predicate_fn = predicate::str::contains("This is a bad string");
    assert_eq!(true, predicate_fn.eval("This is a bad string ralph"));
    assert_eq!(false, predicate_fn.eval("Four Five Six"));
}
