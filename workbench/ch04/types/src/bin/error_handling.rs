use thiserror::Error;

#[derive(Error, Debug)]
enum DivError {
    #[error("Divide by zero: {0}")]
    DivideByZero(i32),
    #[error("Both negative: {0}, {1}")]
    BothNegative(i32, i32),
}

fn func_ex_div_result(x: i32, y: i32) -> Result<i32, DivError> {
    if y == 0 {
        Err(DivError::DivideByZero(y))
    } else if x < 0 && y < 0 {
        Err(DivError::BothNegative(x, y))
    } else {
        Ok(x / y)
    }
}

fn func_ex_div_result2(x: i32, y: i32) -> Result<i32, String> {
    match y {
        0 => Err(String::from("Divide by zero")),
        _ => Ok(x / y),
    }
}

fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str),
    }
}

fn print_mydiv(x: i32, y: i32) {
    match func_ex_div_result(x, y) {
        Ok(res) => println!("no error, ans = {}", res),
        Err(DivError::DivideByZero(y)) => println!("Divide by zero: {}", y),
        Err(DivError::BothNegative(x, y)) => println!("Both negative: {}, {}", x, y),
    }
}

fn print_mydiv2(x: i32, y: i32) {
    match func_ex_div_result(x, y) {
        Ok(res) => println!("no error, ans = {}", res),
        Err(e) => println!("{}", e), // thiserrorにより、詳細なエラー情報が表示される
    }
}

fn mydiv(x: i32, y: i32) -> Result<i32, DivError> {
    if y == 0 {
        Err(DivError::DivideByZero(y))
    } else if x < 0 && y < 0 {
        Err(DivError::BothNegative(x, y))
    } else {
        Ok(x / y)
    }
}

fn repeat_mydiv(ary: &[(i32, i32)]) -> Result<Vec<i32>, DivError> {
    let mut ret: Vec<i32> = Vec::new();
    for aa in ary {
        let ans = mydiv(aa.0, aa.1)?;
        ret.push(ans);
        println!("pushed: {} / {} = {}", aa.0, aa.1, ans);
    }
    Ok(ret)
}

fn print_repeat_mydiv(result: Result<Vec<i32>, DivError>) {
    match result {
        Ok(v) => println!("result: {:?}", v),
        Err(e) => println!("error: {}", e),
    }
}

fn main() {
    let x = 10;
    let y = 2;
    println!("x = {}, y = {}", x, y);
    print_mydiv(x, y);
    print_mydiv2(x, y);
    func_ex_print_result(func_ex_div_result2(x, y));

    let y = 0;
    println!("x = {}, y = {}", x, y);
    print_mydiv(x, y);
    print_mydiv2(x, y);
    func_ex_print_result(func_ex_div_result2(x, y));

    let x = -10;
    let y = -2;
    println!("x = {}, y = {}", x, y);
    print_mydiv(x, y);
    func_ex_print_result(func_ex_div_result2(x, y));
    print_mydiv2(x, y);

    println!("1st calc");
    print_repeat_mydiv(repeat_mydiv(&[(10, 2), (9, 3)]));
    println!("");

    println!("2nd calc");
    print_repeat_mydiv(repeat_mydiv(&[(10, 2), (-9, -3), (5, 2)]));
    println!("");

    println!("3rd calc");
    print_repeat_mydiv(repeat_mydiv(&[(10, 2), (-9, 0), (5, 2)]));
    println!("");
}
