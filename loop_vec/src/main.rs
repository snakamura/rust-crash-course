fn loop_ref() {
    let nums = vec![1, 2, 3, 4, 5];
    for i in 1..3 {
        for j in &nums {
            let _: &u32 = j;
            println!("{},{}", i, j)
        }
    }
}

fn loop_mut_ref() {
    let mut nums = vec![1, 2, 3, 4, 5];
    for i in 1..3 {
        for j in &mut nums {
            let _: &u32 = j;
            let _: &mut u32 = j;
            println!("{},{}", i, j)
        }
    }
}

fn main() {
    loop_ref();
    loop_mut_ref();
}
