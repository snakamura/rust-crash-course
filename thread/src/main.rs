use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

fn sequential() {
    let mut msg: String = String::from("Fearless");
    for _ in 1..11 {
        let mut inner = || {
            msg.push('!');
            println!("{}", msg);
        };
        inner();
        sleep(Duration::new(1, 0));
    }
}

fn concurrent() {
    let msg = Arc::new(Mutex::new(String::from("Fearless")));
    for _ in 1..11 {
        let msg = msg.clone();
        let inner = move || {
            let mut msg = msg.lock().unwrap();
            msg.push('!');
            println!("{}", msg);
        };
        spawn(inner);
        sleep(Duration::new(1, 0));
    }
}

fn main() {
    sequential();
    concurrent();
}
