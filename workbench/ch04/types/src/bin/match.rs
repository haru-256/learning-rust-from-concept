fn print_num(x: i32) {
    println!("x = {}", x);
    let s = match x {
        0 => Some("zero"),
        1 | 2 => Some("small"),
        3..=9 => Some("large"),
        _ => None,
    };

    match s {
        Some(str) => {
            println!("{} is {}", x, str);
        }
        None => {
            println!("{} is not 0, 1, 2, 3, 4, 5, 6, 7, 8 or 9", x);
        }
    }
    println!("");
}

fn main() {
    print_num(0);
    print_num(1);
    print_num(4);
    print_num(8);
    print_num(9);
    print_num(10);
}
