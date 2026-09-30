use std::{
    f64::consts::PI,
    fmt::{Display, Result},
};

/// Trait defining common geometric calculations.
pub trait Shape {
    /// Calculates the surface area of the shape.
    fn area(&self) -> f64;
    /// Calculates the perimeter (outer boundry length) of the shape.
    fn perimeter(&self) -> f64;
}

/// Represents the circle with a given radius.
#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

/// Represents the rectangle with width and height.
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

/// Represents the triangle with side lengths `a`, `b` and `c`.
#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2f64 * PI * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }

    fn perimeter(&self) -> f64 {
        (self.height + self.width) * 2f64
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let sp: f64 = self.perimeter() / 2f64;
        (sp * (sp - self.a) * (sp - self.b) * (sp - self.c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Circle (r = {:.2})", self.radius)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Rectangle (w = {:.2}, h = {:.2})",
            self.width, self.height
        )
    }
}

impl Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Triangle (a = {:.2}, b = {:.2}, c = {:.2})",
            self.a, self.b, self.c
        )
    }
}

/// Print formatted information about a shape to standard output.
pub fn print_shape_info<T: Display + Shape>(shape: &T) {
    print!("--- {} ---", shape);
    print!("Area:       {:.2}", shape.area());
    print!("Perimeter:  {:.2}\n", shape.perimeter());
}

/// Computes the combined total area of a slice of homogeneous shapes using static dispatch.
pub fn total_area<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}
