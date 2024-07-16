// Define a generic struct Point with two type parameters, T and U
struct Point<T, U> {
    x: T,
    y: U,
}

// Implement methods for the generic Point struct
impl<T, U> Point<T, U> {
    // Define a constructor function to create a new Point instance
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

// Implement methods specifically for Point with both T and U as i32
impl Point<i32, i32> {
    // Define a method to print the coordinates of the Point
    fn printing(&self) {
        println!("The value of coordinates are {:?}, {}", self.x, self.y);
    }
}

fn main() {
    // Create a Point instance with both coordinates as i32
    let origin = Point::new(0, 0);

    // Create a Point instance with x as f64 and y as f64
    let p1 = Point::new(1.1, 2.2);

    // Create a Point instance with x as i32 and y as f64
    let p2 = Point::new(5, 5.5);

    // Call the printing method on the origin Point instance
    origin.printing();

    // Note: p1 and p2 are not used further, but they demonstrate the flexibility of generics
}
