fn main() {
    trait Animal {
        fn eat(&self);
    }
    
    struct Herbivore;
    struct Carnivore;
    
    impl Animal for Herbivore {
        fn eat(&self) {
            println!("I eat plants");
        }
    }
    impl Animal for Carnivore {
        fn eat(&self) {
            println!("I eat flesh");
        }
    }
    
    // Create a vector of Animals:
    let mut list: Vec<Box<dyn Animal>> = Vec::new();
    let goat = Herbivore;
    let dog = Carnivore;
    
    list.push(Box::new(goat));
    list.push(Box::new(dog));
    
    // Calling eat() for all animals in the list:
    for animal in &list{
        animal.eat();
    }
}