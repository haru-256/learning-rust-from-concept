trait PrintHello {
    fn print_hello(&self) {
        println!("Hello");
    }
}

// Unlike構造体
struct Test1;
struct Test2;

impl PrintHello for Test1 {}

impl PrintHello for Test2 {
    fn print_hello(&self) {
        println!("Hello, Test2");
    }
}

fn main() {
    let test1 = Test1;
    let test2 = Test2;
    test1.print_hello();
    test2.print_hello();
}
