use std::cmp::Ordering;

fn main() {
    print_sign(determine_sign(10));
    print_sign(determine_sign(-1));
    print_sign(determine_sign(0));

    let mut v: Vec<EnumExample> = Vec::new();
    v.push(EnumExample::TupleTypeExample1(String::from(
        "TupleTypeExample1",
    )));
    v.push(EnumExample::TupleTypeExample2(1, true));
    v.push(EnumExample::StructTypeExample {
        name: String::from("StructTypeExample"),
        age: 1,
    });

    for e in &v {
        if let EnumExample::StructTypeExample { name: n, age: a } = e {
            println!("StructTypeExample: name: {}, age: {}", n, a);
        }
    }

    for e in &v {
        match e {
            EnumExample::TupleTypeExample1(s) => println!("TupleTypeExample1: {}", s),
            EnumExample::TupleTypeExample2(i, b) => println!("TupleTypeExample2: {}, {}", i, b),
            EnumExample::StructTypeExample { name: n, .. } => {
                println!("StructTypeExample: name: {}", n)
            }
        }
    }
}
enum Sign {
    Positive,
    Zero,
    Negative,
}

fn determine_sign(x: i32) -> Sign {
    if x > 0 {
        Sign::Positive
    } else if x < 0 {
        Sign::Negative
    } else {
        Sign::Zero
    }
}

fn determine_sign2(x: i32) -> Sign {
    match x.cmp(&0) {
        Ordering::Greater => Sign::Positive,
        Ordering::Less => Sign::Negative,
        Ordering::Equal => Sign::Zero,
    }
}

fn print_sign(s: Sign) {
    match s {
        Sign::Positive => println!("+"),
        Sign::Negative => println!("0"),
        Sign::Zero => println!("-"),
    }
}

enum EnumExample {
    TupleTypeExample1(String),
    TupleTypeExample2(i32, bool),
    StructTypeExample { name: String, age: u8 },
}
