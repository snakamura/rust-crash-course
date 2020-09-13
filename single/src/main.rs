struct Single<T> {
    value: Option<T>,
}

impl<T> Iterator for Single<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        /*
        let mut value = None;
        std::mem::swap(&mut self.value, &mut value);
        value
        */
        /*
        std::mem::replace(&mut self.value, None)
        */
        self.value.take()
    }
}

fn single<T>(value: T) -> Single<T> {
    Single {
        value: Some(value),
    }
}

fn main() {
    let values: Vec<u32> = single(42).collect();
    assert_eq!(vec![42], values);
}
