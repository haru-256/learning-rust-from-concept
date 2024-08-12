fn main() {
    let ss = "hello, world!";
    println!("{}", ss);
    println!("{}", &ss[0..5]);

    let st = "あいうえお";
    println!("{}", st);
    println!("{}", &st[0..6]);

    let mut st1 = "hello".to_string();
    let st2 = String::from("world");
    st1.push_str(" world");
    println!("{}", st1);
    println!("{}", st2);
    println!("{}", &st1[0..6]);
}
