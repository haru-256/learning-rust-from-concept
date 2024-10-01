trait CalcArea {
    fn calc_area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

struct RightTriangle {
    width: f64,
    height: f64,
}

impl CalcArea for RightTriangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height / 2.0
    }
}

fn area<T: CalcArea>(x: &T) -> f64 {
    x.calc_area()
}
fn area_v2<T>(x: &T) -> f64
where
    T: CalcArea,
{
    x.calc_area()
}

fn main() {
    let rect = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("Area of rectangle: {}", area(&rect));

    let tria = RightTriangle {
        width: 3.0,
        height: 4.0,
    };
    println!("Area of triangle: {}", area_v2(&tria));
}
