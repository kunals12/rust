// Define a struct 'Square' with fields for side length, line width, and color
struct Square {
    side: f32,
    line_width: u8,
    color: String,
}

// Define a struct 'Rectangle' with fields for length, width, line width, and color
struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}

// Define a trait 'Shape' with a method 'area' that returns a float
trait Shape {
    fn area(&self) -> f32;
}

// Implement a constructor for 'Square'
impl Square {
    fn new(side: f32, line_width: u8, color: String) -> Square {
        Square {
            side,
            line_width,
            color,
        }
    }
}

// Implement a constructor for 'Rectangle'
impl Rectangle {
    fn new(length: f32, width: f32, line_width: u8, color: String) -> Rectangle {
        Rectangle {
            length,
            width,
            line_width,
            color,
        }
    }
}

// Implement the 'Shape' trait for 'Square'
impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("Square area {}", area_of_square);
        area_of_square
    }
}

// Implement the 'Shape' trait for 'Rectangle'
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rectangle = self.length * self.width;
        println!("Rectangle area {}", area_of_rectangle);
        area_of_rectangle
    }
}

/*Trait bounds in Rust specify that a generic type parameter must implement a particular trait.
They allow you to enforce constraints on generic types, ensuring that they provide certain behavior or methods.*/
// Define a function that prints the area of a shape, using trait bounds
fn print_area<T: Shape>(shape: &T) {
    println!("Area: {}", shape.area());
}

fn main() {
    // Create a new 'Square' instance
    let s1 = Square::new(5.3, 1, String::from("Red"));

    // Create a new 'Rectangle' instance
    let r1 = Rectangle::new(5.3, 4.3, 1, String::from("Blue"));

    // Calculate and print the area of the square
    s1.area();

    // Calculate and print the area of the rectangle
    r1.area();
    print_area(&s1);
    print_area(&r1);
}
