pub trait Visitor {
    fn visit_person(&self, person: &Person);
    fn visit_organization(&self, o: &Organization);
}

pub trait Element {
    fn accept(&self, visitor: &mut dyn Visitor);
}

impl Element for Person {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_person(self);
    }
}

impl Element for Organization {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_organization(self);
    }
}

pub struct Person {
    name: String,
    email: String,
}

impl Person {
    fn new(name: &str, email: &str) -> Self {
        Self {
            email: email.to_string(),
            name: name.to_string(),
        }
    }
}

pub struct Organization {
    name: String,
    address: String,
}

impl Organization {
    fn new(name: &str, address: &str) -> Self {
        Self {
            name: name.to_string(),
            address: address.to_string(),
        }
    }
}

struct EmailVisitor;

impl Visitor for EmailVisitor {
    fn visit_person(&self, p: &Person) {
        println!("Sending email to {} at {}", p.name, p.email);
    }

    fn visit_organization(&self, o: &Organization) {
        println!("Sending mail to {} at {}", o.name, o.address);
    }
}

fn main() {
    let mut elements: Vec<Box<dyn Element>> = Vec::new();

    let alice = Box::new(Person::new("Alice", "alices@example.com"));
    let bob = Box::new(Person::new("Bob", "bob@example.com"));
    let acme = Box::new(Organization::new("Acme Inc.", "123 Main Str."));

    elements.push(alice);
    elements.push(acme);
    elements.push(bob);

    let mut email_visitor = EmailVisitor;
    for element in elements {
        element.accept(&mut email_visitor);
    }
}
