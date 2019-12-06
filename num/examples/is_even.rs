fn is_even(num: u32) -> bool {
    (num) & 1 == 0
}

fn is_odd(num: u32) -> bool {
    !is_even(num)
}

/*
fn is_odd(self) -> bool {
    !self.is_even()
}
*/

/*
fn test_even() {
    assert_eq!((-4).is_even(), true);
    assert_eq!((-3).is_even(), false);
    assert_eq!((-2).is_even(), true);
    assert_eq!((-1).is_even(), false);
    assert_eq!((0 .is_even(), true);
    assert_eq!((1).is_even(), false);
    assert_eq!((2).is_even(), true);
    assert_eq!((3).is_even(), false);
    assert_eq!((4).is_even(), true);
}
*/

/*
fn test_odd() {
    assert_eq!((-4 as $T).is_odd(), false);
    assert_eq!((-3 as $T).is_odd(), true);
    assert_eq!((-2 as $T).is_odd(), false);
    assert_eq!((-1 as $T).is_odd(), true);
    assert_eq!((0 as $T).is_odd(), false);
    assert_eq!((1 as $T).is_odd(), true);
    assert_eq!((2 as $T).is_odd(), false);
    assert_eq!((3 as $T).is_odd(), true);
    assert_eq!((4 as $T).is_odd(), false);
}
*/

fn main() {
    let x0 = is_even(0);
    let x1 = is_even(1);
    let x2 = is_even(2);
    let x3 = is_even(3);
    println!("{} {} {} {}", x0, x1, x2, x3);
    let y0 = is_odd(0);
    let y1 = is_odd(1);
    let y2 = is_odd(2);
    let y3 = is_odd(3);
    println!("{} {} {} {}", y0, y1, y2, y3);
}
