fn main() {
    let ss = "hello, world!";
    println!("{}", ss);
    println!("{}", &ss[0..5]);

    let st = "あいうえお";
    println!("{}", st);
    println!("{}", &st[0..6]);
    let o = st.chars().nth(2);
    match o {
        Some(c) => println!("{}", c),
        None => println!("None"),
    }

    let mut st1 = "hello".to_string();
    let st2 = String::from("world");
    st1.push_str(" world");
    println!("{}", st1);
    println!("{}", st2);
    println!("{}", &st1[0..6]);
    println!("{}", st2.chars().take(5).collect::<String>());
    println!("{}", st2.chars().skip(1).take(5).collect::<String>());
}
