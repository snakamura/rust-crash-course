use std::fmt::Display;

struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "Name: {}, Arge: {}", self.name, self.age)
    }
}

fn main() {
    let person = Person {
        name: String::from("Scott Tiger"),
        age: 12,
    };

    println!("Person: {}", person)
}
