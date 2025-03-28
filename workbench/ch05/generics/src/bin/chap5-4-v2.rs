#[derive(Debug)]
struct Point2d<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
{
    x: T,
    y: T,
}
impl<T> Point2d<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
{
    fn new(x: T, y: T) -> Self {
        Point2d { x, y }
    }

    fn distance_sq(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

fn main() {
    let p1 = Point2d::new(1.0, 2.0);
    let p2 = Point2d { x: 3, y: 4 };
    println!("p1: {:?}, distance: {:?}", p1, p1.distance_sq());
    println!("p2: {:?}, distance: {:?}", p2, p2.distance_sq());
}
