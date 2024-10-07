use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::{Debug, Display};
use std::ops::Neg;

trait IAbs {
    type Output;
    fn iabs(self) -> <Self as IAbs>::Output
    where
        Self:
            Sized + PartialOrd + Neg + From<i8> + TryInto<<Self as IAbs>::Output> + Debug + Display,
        <Self as IAbs>::Output: TryFrom<<Self as Neg>::Output> + Debug + Display,
        <Self as TryInto<<Self as IAbs>::Output>>::Error: Debug,
        <<Self as IAbs>::Output as TryFrom<<Self as Neg>::Output>>::Error: Debug,
    {
        if self < (0_i8).into() {
            self.try_into().unwrap()
        } else {
            (-self).try_into().unwrap()
        }
    }
}

impl IAbs for i32 {
    type Output = u32;
}

impl IAbs for i8 {
    type Output = u8;
}

impl IAbs for i16 {
    type Output = u16;
}

impl IAbs for i64 {
    type Output = u64;
}

fn print_abs<T: IAbs>(x: T) {
    println!("Absolute value of {} is {}", x, x.iabs());
}

fn main() {
    let a = -10;
    println!("Absolute value of {} is {}", a, a.iabs());
}
