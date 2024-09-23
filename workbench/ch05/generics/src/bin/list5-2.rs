struct GenEx<T> {
    value: T,
}

impl<T> GenEx<T>e{
    fn get_value(self) -> T {
        self.value
    }
}

fn main() {
    let x1 = GenEx { value: 42 };
    let x2 = GenEx {
        value: String::from("Hello"),
    };
    let x3 = GenEx { value: 3.14 };
    let x3 = GenEx::<f32> { value: 3.14 };
    println!("x1: {}", x1.get_value());
    println!("x2: {}", x2.get_value());
    println!("x3: {}", x3.get_value());
}
