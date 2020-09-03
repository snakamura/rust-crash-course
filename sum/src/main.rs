fn sum<I>(iter: I) -> I::Item
where
    I: Iterator,
    I::Item: std::ops::Add<Output = I::Item> + From<u8>,
{
    iter.fold(From::from(0), std::ops::Add::add)
}

fn main() {
    let s = (1..=10).fold(0, |a, v| a + v);
    println!("{}", s);

    println!("{}", sum(1..=10))
}
