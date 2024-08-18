use crate::RecursiveEnum::{Null, Val};

#[derive(Debug)]
enum RecursiveEnum {
    Val(Box<RecursiveEnum>),
    Null,
}

fn main() {
    let boxed = Box::new(5);
    let val = *boxed;

    println!("val: {}", val);

    let x = Val(Box::new(Val(Box::new(Val(Box::new(Null))))));
    match x {
        Val(y) => println!("x is Val: {:?}", y),
        Null => println!("x is Null"),
    }
}
