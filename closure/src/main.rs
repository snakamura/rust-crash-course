fn call_fn<F, R>(f: F) -> R
where
    F: Fn() -> R,
{
    f()
}

fn call_fn_mut<F, R>(mut f: F) -> R
where
    F: FnMut() -> R,
{
    f()
}

fn call_fn_once<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    f()
}

fn by_ref() {
    let name = String::from("Alice");
    let say_hi = || println!("Hello, {}", name);

    call_fn(say_hi);
    call_fn_mut(say_hi);
    call_fn_once(say_hi);
}

fn by_value() {
    let name = String::from("Alice");
    let say_hi = || {
        let name = name;
        println!("Hello, {}", name);
    };

//    call_fn(say_hi);
//    call_fn_mut(say_hi);
    call_fn_once(say_hi);
}

fn by_ref_move() {
    let say_hi = {
        let name = String::from("Alice");
        move || println!("Hello, {}", name)
    };

    call_fn(&say_hi);
    call_fn_mut(&say_hi);
    call_fn_once(&say_hi);
}

fn by_ref_mut() {
    let mut name = String::from("Alice");
    let mut say_hi = || {
        name += " and Bob";
        println!("Hello, {}", name);
    };

//    call_fn(say_hi);
    call_fn_mut(&mut say_hi);
    call_fn_once(&mut say_hi);

    println!("And now name is: {}", name)
}

fn by_ref_mut2() {
    let mut say_hi = {
        let mut name = String::from("Alice");
        move || {
            name += " and Bob";
            println!("Hello, {}", name);
        }
    };

//    call_fn(say_hi);
    call_fn_mut(&mut say_hi);
    call_fn_once(&mut say_hi);
}

fn main() {
    by_ref();
    by_value();
    by_ref_move();
    by_ref_mut();
    by_ref_mut2();
}
