fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let filtered: Vec<i32> = v.into_iter().filter(|&x| x >= 2).collect();
    println!("{:?}", filtered);

    let w = vec![4, 5, 6];
    let sum = w.into_iter().fold(1, |acc, x| acc + x);
    println!("{}", sum);
    let w = vec![4, 5, 6]; // into_iter() consumes the vector, so we need to create a new one
    let sum = w.into_iter().reduce(|acc, x| acc + x).unwrap();
    println!("{:?}", sum);
}
