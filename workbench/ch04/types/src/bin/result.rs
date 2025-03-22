use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use thiserror::Error;

fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("div by zero")
    } else {
        Ok(x / y)
    }
}

fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
}

#[derive(Error, Debug)]
enum DivError {
    #[error("div by zero: {0}")]
    DivByZero(i32),
    #[error("both negative: {0}, {1}")]
    BothNegative(i32, i32),
}

fn my_div(x: i32, y: i32) -> Result<i32, DivError> {
    if y == 0 {
        Err(DivError::DivByZero(x))
    } else if x < 0 && y < 0 {
        Err(DivError::BothNegative(x, y))
    } else {
        Ok(x / y)
    }
}

fn print_mydiv(x: i32, y: i32) {
    match my_div(x, y) {
        Ok(v) => println!("{}", v),
        Err(DivError::DivByZero(v)) => println!("div by zero: {}", v),
        Err(DivError::BothNegative(v1, v2)) => println!("both negative: {}, {}", v1, v2),
    }
}

fn print_mydiv2(x: i32, y: i32) {
    match my_div(x, y) {
        Ok(v) => println!("no error. ans = {}", v),
        Err(e) => println!("{}", e),
    }
}

fn print_mydiv3(x: i32, y: i32) {
    let ret = my_div(x, y);
    if ret.is_ok() {
        println!("no error. ans = {}", ret.unwrap());
    } else {
        println!("{}", ret.err().unwrap());
    }
}

fn repeat_mydiv(ary: &[(i32, i32)]) -> Result<Vec<i32>, DivError> {
    let mut ret = Vec::new();
    for aa in ary {
        ret.push(my_div(aa.0, aa.1)?);
    }
    // for (x, y) in ary {
    //     ret.push(my_div(*x, *y));
    // }
    Ok(ret)
}

fn print_repeat_mydiv(ans: Result<Vec<i32>, DivError>) {
    match ans {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    func_ex_print_result(func_ex_div_result(10, 2));
    func_ex_print_result(func_ex_div_result(10, 0));

    print_mydiv(10, 2);
    print_mydiv(10, 0);
    print_mydiv(-10, -2);

    print_mydiv2(10, 2);
    print_mydiv2(10, 0);
    print_mydiv2(-10, -2);

    print_mydiv3(10, 2);
    print_mydiv3(10, 0);
    print_mydiv3(-10, -2);

    println!("1st calc");
    print_repeat_mydiv(repeat_mydiv(&[(10, 2), (10, 1), (10, -2)]));
    println!("2nd calc");
    print_repeat_mydiv(repeat_mydiv(&[(10, 2), (10, 0), (-10, -2)]));
    println!("3rd calc");
    print_repeat_mydiv(repeat_mydiv(&[(10, 2), (-10, -2)]));

    let f = File::open("./input.txt")?;
    let f = BufReader::new(f);
    for line in f.lines().flatten() {
        let mut v = Vec::new();
        for ee in line.split(" ") {
            v.push(ee.parse()?);
        }
        let result = my_div(v[0], v[1])?;
        println!("{} / {} = {}", v[0], v[1], result);
    }

    Ok(())
}
