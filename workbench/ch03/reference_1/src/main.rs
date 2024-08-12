fn myprint<T: std::fmt::Display>(msg: &T) {
    println!("{}", *msg);
}

fn main() {
    let s = "Hello, world!".to_string();
    let s_ref = &s;
    let s_ref2 = &s;
    myprint(s_ref);
    myprint(s_ref);

    myprint(s_ref2);
    myprint(s_ref2);
}
