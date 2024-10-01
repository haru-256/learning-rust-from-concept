trait CalcLength {
    fn calc_length(&self) -> f64;
}

trait CalcArea {
    fn calc_area(&self) -> f64;
}

struct Line {
    length: f64,
}

impl CalcLength for Line {
    fn calc_length(&self) -> f64 {
        self.length
    }
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

impl CalcLength for Rectangle {
    fn calc_length(&self) -> f64 {
        2.0 * (self.width + self.height)
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

impl CalcLength for RightTriangle {
    fn calc_length(&self) -> f64 {
        self.width + self.height + (self.width.powi(2) + self.height.powi(2)).sqrt()
    }
}

fn length<T: CalcLength>(x: &T) -> f64 {
    x.calc_length()
}

fn area<T: CalcArea>(x: &T) -> f64 {
    x.calc_area()
}

fn main() {
    let line = Line { length: 5.0 };
    println!("rectangle: length={}", length(&line));

    let rect = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("rectangle: area={}, length={}", area(&rect), length(&rect));

    let tria = RightTriangle {
        width: 3.0,
        height: 4.0,
    };
    println!("triangle: area={}, length={}", area(&tria), length(&tria));
}
