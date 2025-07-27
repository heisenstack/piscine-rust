#[derive(Debug, Clone, Copy)]

pub struct Point(pub f64, pub f64);
const PI: f64 = std::f64::consts::PI;


impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx * dx + dy * dy).sqrt()
    }
}
#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
   pub fn new(x: f64, y: f64, radius: f64) -> Self {
    Self {center:Point(x, y) , radius: radius }
   }
    pub fn area(&self) -> f64 {
        PI  * self.radius * self.radius 
    }
    pub fn diameter(&self) -> f64 {
        2 as f64 * self.radius
    }
    pub fn intersect(&self, other: Circle) -> bool {
        self.center.distance(other.center) <= self.radius + other.radius
    }
}

// use circle::*;

fn main() {
    let circle = Circle::new(500.0, 500.0, 150.0);
    let circle1 = Circle {
        center: Point(80.0, 115.0),
        radius: 30.0,
    };
    // println!("{}", PI_F32);

    // println!("{:#?}", circle1);
    // println!("{:#?}", circle);

    let point_a = Point(1.0, 1.0);
    let point_b = Point(0.0, 0.0);
    println!("circle = {:?} area = {}", circle, circle.area());
    println!("circle = {:?} diameter = {}", circle, circle.diameter());
    println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
    println!("circle and circle1 intersect = {}", circle.intersect(circle1));

    println!("distance between {:?} and {:?} is {}", point_a, point_b, point_a.distance(point_b));
}
