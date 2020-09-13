#[derive(Debug)]
struct Person {
    name: &'static str,
    age: u32,
}

const ALICE: Person = Person {
    name: "Alice",
    age: 18,
};

fn get_alice() -> &'static Person {
    &ALICE
}

fn main() {
    let alice = get_alice();
    println!("{:?}", alice)
}
