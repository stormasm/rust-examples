use average::AverageCollection;

fn main() {
    println!("Hello, world!");

    let mut p1 = AverageCollection::default();
    println!("average = {}", p1.average());

    p1.add(1);
    println!("average = {}", p1.average());

    p1.add(3);
    println!("average = {}", p1.average());

    p1.add(5);
    println!("average = {}", p1.average());

    p1.add(7);
    println!("average = {}", p1.average());

    p1.remove();
    println!("average = {}", p1.average());
}
