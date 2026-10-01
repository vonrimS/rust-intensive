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
    println!("--- {} ---", shape);
    println!("Area:       {:.2}", shape.area());
    println!("Perimeter:  {:.2}\n", shape.perimeter());
}

/// Computes the combined total area of a slice of homogeneous shapes using static dispatch.
pub fn total_area<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_circle_area_and_perimeter() {
        let circle = Circle { radius: 5.0 };
        assert!((circle.area() - (PI * 25.0)).abs() < 1e-6);
        assert!((circle.perimeter() - (PI * 10.0)).abs() < 1e-6);
    }

    #[test]
    fn test_rectangle_area_and_perimeter() {
        let rectangle = Rectangle {
            width: 4.0,
            height: 6.0,
        };
        assert_eq!(rectangle.area(), 24.0);
        assert_eq!(rectangle.perimeter(), 20.0);
    }

    #[test]
    fn test_triangle_heron_formula() {
        let triangle = Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        };
        assert!((triangle.area() - 6.0).abs() < 1e-6);
        assert_eq!(triangle.perimeter(), 12.0);
    }

    #[test]
    fn test_total_area_aggregate() {
        let rects = vec![
            Rectangle {
                width: 4.0,
                height: 6.0,
            },
            Rectangle {
                width: 4.0,
                height: 6.0,
            },
        ];

        assert_eq!(total_area(&rects), 48.0);
    }

    #[test]
    fn test_zero_edge_cases() {
        let zero_circle = Circle { radius: 0.0 };
        assert_eq!(zero_circle.area(), 0.0);
        assert_eq!(zero_circle.perimeter(), 0.0);

        let zero_rectangle = Rectangle {
            width: 0.0,
            height: 0.0,
        };
        assert_eq!(zero_rectangle.area(), 0.0);
        assert_eq!(zero_rectangle.perimeter(), 0.0);

        let zero_triangle = Triangle {
            a: 0.0,
            b: 0.0,
            c: 0.0,
        };
        assert_eq!(zero_triangle.area(), 0.0);
        assert_eq!(zero_triangle.perimeter(), 0.0);

        let empty_shapes: [Rectangle; 0] = [];
        assert_eq!(total_area(&empty_shapes), 0.0);
    }

    #[test]
    fn test_display_formatting() {
        let circle = Circle { radius: 5.0 };
        let rectangle = Rectangle {
            width: 4.0,
            height: 6.0,
        };
        let triangle = Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        };

        assert_eq!(format!("{}", circle), "Circle (r = 5.00)");
        assert_eq!(format!("{}", rectangle), "Rectangle (w = 4.00, h = 6.00)");
        assert_eq!(
            format!("{}", triangle),
            "Triangle (a = 3.00, b = 4.00, c = 5.00)"
        );
    }

    #[test]
    fn test_derived_traits() {
        let circle1 = Circle { radius: 5.0 };
        let circle2 = circle1.clone();

        assert_eq!(circle1, circle2);
    }
}
