trait MyError: std::fmt::Debug {}

#[derive(Debug)]
struct MyError1;
impl MyError for MyError1 {}

#[derive(Debug)]
struct MyError2;
impl MyError for MyError2 {}

#[derive(Debug)]
struct MyError3;
impl MyError for MyError3 {}

#[derive(Debug)]
struct MyErrorOther;
impl MyError for MyErrorOther {}

fn div4(x: i32) -> Result<i32, Box<dyn MyError>> {
    let res = x % 4;
    match res {
        0 => Ok(res),
        1 => Err(Box::new(MyError1)),
        2 => Err(Box::new(MyError2)),
        3 => Err(Box::new(MyError3)),
        _ => Err(Box::new(MyErrorOther)),
    }
}

fn main() {
    match div4(0) {
        Ok(v) => println!("div4(0) = {:?}", v),
        Err(e) => println!("div4(0) = {:?}", e),
    }

    println!("div4(0) = {:?}", div4(0));
    println!("div4(1) = {:?}", div4(1));
    println!("div4(2) = {:?}", div4(2));
    println!("div4(3) = {:?}", div4(3));
    println!("div4(4) = {:?}", div4(4));

    let mut v = Vec::<Box<dyn std::fmt::Debug>>::new();
    v.push(Box::new(MyError1));
    v.push(Box::new(1_i32));
    v.push(Box::new(String::from("hello")));

    println!("v = {:?}", v);
}
