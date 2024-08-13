fn main() {
    let x = 1;
    let x_ptr: *const i32 = &x;

    println!("*x_ptr: {}", unsafe { *x_ptr });

    let mut y = 2;
    // let y_ptr: *mut i32 = &mut y;
    let y_ptr = &mut y as *mut i32;
    unsafe {
        *y_ptr = *x_ptr + 4;
    }
    println!("*y_ptr: {}", unsafe { *y_ptr });
    println!("y: {}", y);
}
