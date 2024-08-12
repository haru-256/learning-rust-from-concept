fn myprint<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}

fn main() {
    let s = 1;
    myprint(s);
    myprint(s); // sをもう一回出力

    let s = "hello".to_string();
    let ss = s.clone();
    myprint(s);
    // myprint(s);
    myprint(ss);
}
