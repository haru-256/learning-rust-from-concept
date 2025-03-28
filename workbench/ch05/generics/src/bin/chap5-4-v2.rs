use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::ops::Add;
use std::ops::Mul;
#[derive(Debug, Clone, Copy)]
struct Point2d<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    x: T,
    y: T,
}
impl<T> Point2d<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    fn new(x: T, y: T) -> Self {
        Point2d { x, y }
    }

    fn distance_sq(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl<T> Add for Point2d<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> PartialEq for Point2d<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let dist_self_sq = self.distance_sq();
        let dist_other_sq = other.distance_sq();
        dist_self_sq.eq(&dist_other_sq)
    }
}
impl<T> PartialOrd for Point2d<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let distance1 = self.distance_sq();
        let distance2 = other.distance_sq();
        distance1.partial_cmp(&distance2)
    }
}

fn main() {
    let p1 = Point2d::new(1.0, 2.0);
    let p2 = Point2d { x: 3, y: 4 };
    let p3 = Point2d { x: 3, y: 4 };
    println!("p1: {:?}, distance: {:?}", p1, p1.distance_sq());
    println!("p2: {:?}, distance: {:?}", p2, p2.distance_sq());
    println!("p3: {:?}, distance: {:?}", p3, p3.distance_sq());
    println!("p2 + p3: {:?}", p2 + p3);
    println!("p2 == p3? : {:?}", p2 == p3);
    println!("p2 > p3? : {:?}", p2 > p3);
}
