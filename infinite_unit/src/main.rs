struct InfiniteUnit;

impl IntoIterator for InfiniteUnit {
    type Item = ();
    type IntoIter = InfiniteUnitIterator;
    fn into_iter(self) -> Self::IntoIter {
        InfiniteUnitIterator
    }
}

struct InfiniteUnitIterator;

impl Iterator for InfiniteUnitIterator {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        Some(())
    }
}

struct InfiniteUnitRepeat;

impl IntoIterator for InfiniteUnitRepeat {
    type Item = ();
    type IntoIter = std::iter::Repeat<()>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::repeat(())
    }
}

fn print_iterator<I: IntoIterator>(iter: I) {
    let mut count = 0;
    for _ in iter {
        count += 1;
        println!("count == {}", count);
        if count >= 5 {
            break;
        }
    }
}

fn main() {
    print_iterator(InfiniteUnit);
    print_iterator(InfiniteUnitRepeat);
}
