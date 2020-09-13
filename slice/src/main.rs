use std::ops::Deref;

struct MyArray([u32; 5]);

impl Deref for MyArray {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn print_slice(slice: &[u32]) {
    for v in slice {
        println!("{}", v);
    }
}

fn main() {
    let array: [u32; 5] = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];
    let my_array = MyArray([1, 2, 3, 4, 5]);

    print_slice(&array);
    print_slice(&vec);
    print_slice(&my_array);
}
