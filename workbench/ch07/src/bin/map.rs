fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5].into_iter().map(add_one).collect();
    println!("{:?}", v);

    let w: Vec<i32> = vec![4, 5, 6].into_iter().map(|x| x + 1).collect();
    let w: Vec<i32> = vec![4, 5, 6]
        .into_iter()
        .map(|x: i32| -> i32 { x + 1 })
        .collect();
    println!("{:?}", w);

    let mut m = 1;
    let f1 = |x: i32| -> i32 { x + m };
    println!("{:?}", f1(1));

    m = 2;
    // println!("{:?}", f1(1)); // This will cause a compile error because `f1` captures `m` by reference
    let f2 = |x: i32| -> i32 { x + m };
    println!("{:?}", f2(1));

    let m = 3;
    let f3 = |x: i32| -> i32 { x + m };
    println!("{:?}", f3(1));
}
