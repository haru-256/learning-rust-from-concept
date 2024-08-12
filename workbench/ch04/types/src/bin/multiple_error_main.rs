use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use thiserror::Error;

// Box: https://zenn.dev/woden/articles/46e0b4a0c8264d
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("./input.txt")?;
    let f = BufReader::new(f);

    for line in f.lines().flatten() {
        let mut v = Vec::new();
        for ee in line.split(' ') {
            // v.push(ee.parse::<i32>()?); // turbofishで指定しなくとも自動的に推論してくれる
            v.push(ee.parse()?);
        }
        let result = mydiv(v[0], v[1])?;
        println!("v := {:?}, {} / {} = {}", v, v[0], v[1], result);
    }

    Ok(())
}

#[derive(Error, Debug)]
enum DivError {
    #[error("Divide by zero: {0}")]
    DivideByZero(i32),
    #[error("Both negative: {0}, {1}")]
    BothNegative(i32, i32),
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
