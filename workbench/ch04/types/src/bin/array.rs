fn main() {
    let ary = [1, 2, 3, 4, 5];
    println!("array");
    for aa in &ary {
        println!("{}", aa);
    }
    let ary_sliced = &ary[1..4];
    println!("array slice");
    for aa in ary_sliced {
        println!("{}", aa);
    }

    let mut v = vec![1, 2, 3, 4, 5];

    println!("before push: {:?}", v);
    v.push(10);
    println!("after push: {:?}", v);
    v[2] += 10;
    println!("after v[2] += 10: {:?}", v);
    println!("&v[3...]: {:?}", &v[3..]);
}
