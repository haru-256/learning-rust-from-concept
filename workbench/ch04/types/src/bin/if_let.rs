enum OddEven {
    Odd,
    Even,
}

enum OddEven2 {
    Odd(i32),
    Even(i32),
}

fn odd_or_even(x: i32) -> OddEven {
    if x % 2 == 0 {
        OddEven::Even
    } else {
        OddEven::Odd
    }
}

fn print_only_even(x: i32) {
    if let OddEven::Even = odd_or_even(x) {
        println!("{} is even", x);
    }
}

fn odd_or_even2(x: i32) -> OddEven2 {
    if x % 2 == 0 {
        OddEven2::Even(x)
    } else {
        OddEven2::Odd(x)
    }
}

fn print_only_even2(x: i32) {
    if let OddEven2::Even(y) = odd_or_even2(x) {
        println!("{} is even: {}", x, y);
    }
    if let OddEven2::Odd(y) = odd_or_even2(x) {
        println!("{} is odd: {}", x, y);
    }
}

fn main() {
    print_only_even(2);
    print_only_even(3);
    print_only_even2(4);
    print_only_even2(5);
}
