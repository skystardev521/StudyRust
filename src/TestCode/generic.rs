use std::fmt::Debug;

//pub mod Gneric{
trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

pub fn test_trait() {
    let c = Circle {
        x: 0.11f64,
        y: 0.99f64,
        radius: 1.99f64,
    };

    let s = Square {
        x: 1.11f64,
        y: 2.99f64,
        side: 3.99f64,
    };

    print_area(c);
    print_area(s);
}

struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

///
/// no use where
///
pub fn foo<T: Clone, K: Clone + std::fmt::Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

///
/// use where
///
fn bar<T, K>(x: T, y: K)
where
    T: Clone,
    K: Clone + Debug,
{
    x.clone();
    y.clone();
    println!("{:?}", y);
}

trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 {
        *self as i64
    }
}

fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}

fn normal1<T>(x: T) -> T
where
    T: ConvertTo<T>,
{
    x.convert()
}

//}
