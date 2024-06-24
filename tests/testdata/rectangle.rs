use std::ops::{Add, Mul};

#[derive(Debug)]
struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T> Rectangle<T> {
    fn new(width: T, height: T) -> Self {
        Rectangle { width, height }
    }
}

// Adding a method to calculate the area, requiring T to implement Mul and Copy
impl<T> Rectangle<T>
where
    T: Mul<Output = T> + Copy,
{
    fn area(&self) -> T {
        self.width * self.height
    }
}

// Adding a method to calculate the perimeter, requiring T to implement Add and Copy
impl<T> Rectangle<T>
where
    T: Add<Output = T> + Copy,
{
    fn perimeter(&self) -> T {
        (self.width + self.height) + (self.width + self.height)
    }
}

fn main() {
    let rect1 = Rectangle::new(3, 4); // Rectangle with integers
    let rect2 = Rectangle::new(3.5, 4.5); // Rectangle with floats

    println!("Rectangle 1: {:?}", rect1);
    println!("Area: {}", rect1.area());
    println!("Perimeter: {}", rect1.perimeter());

    println!("Rectangle 2: {:?}", rect2);
    println!("Area: {}", rect2.area());
    println!("Perimeter: {}", rect2.perimeter());
}
