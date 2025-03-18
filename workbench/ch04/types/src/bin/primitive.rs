fn main() {
    let ii: i64 = 0;
    println!("ii: {}", ii);
    let ii = 0_32;
    println!("ii: {}", ii);

    let ii = 1_i8;
    let jj = 2_u8;
    let kk = ii as i32 + jj as i32;
    println!("kk: {}", kk);
}
