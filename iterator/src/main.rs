use std::fmt::Display;

struct Empty;

impl Iterator for Empty {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

struct Ten {
    value: u32,
}

impl Ten {
    fn new() -> Ten {
        Ten { value: 0 }
    }
}

impl Iterator for Ten {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value < 10 {
            let value = self.value;
            self.value += 1;
            Some(value)
        } else {
            None
        }
    }
}

struct Fib(u32, u32);

impl Fib {
    fn new() -> Fib {
        Fib(0, 1)
    }
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let v0 = self.0;
        let v1 = self.1;

        self.0 = v1;
        self.1 = v0 + v1;

        Some(v0)
    }
}

struct Doubler<I> {
    iter: I,
}

impl<I> Doubler<I> {
    fn new(iter: I) -> Doubler<I> {
        Doubler { iter }
    }
}

impl<I> Iterator for Doubler<I>
where
    I: Iterator,
    I::Item: std::ops::Mul<Output = I::Item> + From<u8>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(n) => Some(n * From::from(2)),
            _ => None,
        }
    }
}

struct Doubler2<I> {
    iter: I,
}

impl<I> Iterator for Doubler2<I>
where
    I: Iterator,
    I::Item: std::ops::Add<Output = I::Item> + Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(n) => Some(n + n),
            None => None,
        }
    }
}

fn print_iterator<I>(name: &str, iter: &mut I)
where
    I: Iterator,
    I::Item: Display,
{
    println!("{}", name);
    for n in iter {
        println!("{}", n);
    }
    println!("");
}

fn main() {
    print_iterator("Empty", &mut Empty);
    print_iterator("Ten", &mut Ten::new());
    print_iterator("Fib", &mut Fib::new().take(10));
    print_iterator("Doubler", &mut Doubler::new(Ten::new()));
    print_iterator("Doubler2", &mut Doubler2 { iter: Ten::new() });
}
