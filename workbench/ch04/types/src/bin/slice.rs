fn main() {
    let ary = [1, 2, 3, 4, 5];
    let ary_sliced = &ary[1..3];
    println!("array slice: {:?}", ary_sliced);
}
