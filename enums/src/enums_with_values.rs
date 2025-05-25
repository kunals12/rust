use std::f32::consts::PI;

enum Shape {
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32),
}

fn main() {
    let shape_square = Shape::Square(10.0);
    let shape_circle = Shape::Circle(10.0);
    let shape_rect = Shape::Rectangle(10.0, 8.0);
    let area = print_area(shape_circle);
    println!("{}", area);
}

fn print_area(shape: Shape) -> f32 {
    match shape {
        Shape::Square(side) => side * side * side * side,
        Shape::Rectangle(side1, side2) => (side1 + side2) * (side1 + side2),
        Shape::Circle(radius) => PI * radius * radius,
    }
}
