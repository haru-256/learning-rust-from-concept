use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Point2d {
    x: f64,
    y: f64,
}

impl Point2d {
    fn new(x: f64, y: f64) -> Self {
        Point2d { x, y }
    }
    fn distance_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }
}

impl Add for Point2d {
    type Output = Self; // Self is Point2d

    fn add(self, rhs: Self) -> <Self as Add>::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl PartialEq for Point2d {
    fn eq(&self, other: &Self) -> bool {
        let dist_self_sq = self.distance_sq();
        let dist_other_sq = other.distance_sq();
        dist_self_sq.eq(&dist_other_sq)
    }
}

impl PartialOrd for Point2d {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let distance1 = self.distance_sq();
        let distance2 = other.distance_sq();
        distance1.partial_cmp(&distance2)
    }
}

#[derive(Debug)]
struct Point2dInt {
    x: i64,
    y: i64,
}

impl Point2dInt {
    fn new(x: i64, y: i64) -> Self {
        Point2dInt { x, y }
    }

    fn distance_sq(&self) -> i64 {
        self.x.pow(2) + self.y.pow(2)
    }
}

impl Add for Point2dInt {
    type Output = Self;

    // Thus, <Self as Add>::Output is precisely the same as writing just Self, because you’ve already defined type Output = Self;.
    fn add(self, rhs: Self) -> <Self as Add>::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl PartialEq for Point2dInt {
    fn eq(&self, other: &Self) -> bool {
        let dist_self_sq = self.distance_sq();
        let dist_other_sq = other.distance_sq();
        dist_self_sq.eq(&dist_other_sq)
    }
}

impl Eq for Point2dInt {}

impl PartialOrd for Point2dInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point2dInt {
    fn cmp(&self, other: &Self) -> Ordering {
        let distance1 = self.distance_sq();
        let distance2 = other.distance_sq();
        distance1.cmp(&distance2)
    }
}

fn main() {
    let x = Point2d::new(3.0, 4.0);
    let y = Point2d { x: 5.0, y: 12.0 };
    let z = Point2d { x: 3.0, y: 4.0 };
    println!("x + y : {:?}", x + y);
    println!("x == y? : {:?}", x == y);
    println!("x > z? : {:?}", x > z);

    let x = Point2dInt::new(3, 4);
    let y = Point2dInt { x: 5, y: 12 };
    let z = Point2dInt { x: 3, y: 4 };
    println!("x : {:?}", x);
    println!("x distance_sq : {:?}", x.distance_sq());
    println!("x == y? : {:?}", x == y);
    println!("x > z? : {:?}", x > z);
}
