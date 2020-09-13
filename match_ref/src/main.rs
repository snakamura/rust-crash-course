#[derive(Debug)]
struct Person {
    name: Option<String>,
    age: Option<u32>,
}

fn print_person(person: Person) {
    match person.name {
        Some(ref name) => println!("Name is {}", name),
        None => println!("No name"),
    }

    match person.age {
        Some(age) => println!("Age is {}", age),
        None => println!("No age"),
    }

    println!("{:?}", person)
}

fn birthday(mut person: Person) {
    match person.age {
        Some(ref mut age) => *age += 1,
        None => (),
    }

    println!("{:?}", person)
}

fn main() {
    print_person(Person {
        name: Some(String::from("Alice")),
        age: Some(30),
    });

    birthday(Person {
        name: Some(String::from("Alice")),
        age: Some(30),
    })
}
