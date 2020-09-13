struct Person {
    name: String,
    age: u32
}

fn older<'a>(person1: &'a Person, person2: &'a Person) -> &'a Person {
    if person1.age >= person2.age {
        person1
    } else {
        person2
    }
}

fn print_older(person: &Person) {
    let bob = Person {
        name: String::from("Bob"),
        age: 32,
    };

    let older_person = older(&person, &bob);
    println!("{}", older_person.name);
}

fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 18,
    };
    print_older(&alice);
}
