fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn func_ex_div_some2(x: i32, y: i32) -> Option<i32> {
    match y {
        0 => None,
        _ => Some(x / y),
    }
}

fn main() {
    let x = 10;
    let y = 2;
    if let Some(z) = func_ex_div_some(x, y) {
        println!("{}", z);
    } else {
        println!("error");
    }
    match func_ex_div_some2(x, y) {
        Some(z) => println!("{}", z),
        None => println!("error"),
    }
}
