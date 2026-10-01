use p9_shape_calculator::{Circle, Rectangle, Triangle, print_shape_info};

fn main() {
    println!("=== Geometric Shape Calculator ===\n");

    let circle = Circle { radius: 5.0 };
    print_shape_info(&circle);

    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };
    print_shape_info(&rectangle);

    let triangle = Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    };
    print_shape_info(&triangle);
}
