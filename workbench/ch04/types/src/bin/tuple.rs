fn main() {
    let tup = (1, 2.5, true);
    println!("tuple");
    println!("{:?}", tup);
    println!("0: {}, 1: {}, 2: {}", tup.0, tup.1, tup.2);

    let (a, b, c) = tup; // unpacking
    println!("a: {}, b: {}, c: {}", a, b, c);
}
