fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    println!("before push: {:?}", v);
    v.push(10);
    println!("after push: {:?}", v);
    v[2] += 10;
    println!("after v[2] += 10: {:?}", v);
    println!("&v[3...]: {:?}", &v[3..]);

    let v2 = vec![100, 200, 300];
    v.extend(v2);
    println!("after extend: {:?}", v);
}
