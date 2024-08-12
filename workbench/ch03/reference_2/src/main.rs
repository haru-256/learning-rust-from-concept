fn myclear(x: &mut String) {
    x.clear();
}

// fn return_hello() -> &String {
//     let s = "hello".to_string();
//     return &s;
// }

fn main() {
    let mut s = "hello".to_string();
    println!("s = {}", s);
    s.clear();
    println!("s = {}", s);

    let mut s = "hello".to_string();
    println!("s = {}", s);

    let s_ref = &mut s;
    myclear(s_ref);
    println!("s = {}", s);

    let s_ref2 = &mut s;
    myclear(s_ref2);
    println!("s = {}", s);

    let mut x = 1;
    let x_ref = &mut x;

    // x = 2; // error: cannot assign to `x` because it is borrowed
    println!("x = {}", x_ref);

    // let s = return_hello();
    // println!("s = {}", s);
}
