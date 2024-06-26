// Define a module named `Shapes`
mod Shapes {
    // Define a public struct `Circle`
    pub struct Circle {
        // `radius` is a private field of the struct
        radius: f32,
    }

    // Implement methods for the `Circle` struct
    impl Circle {
        // Constructor method to create a new `Circle` instance
        // The method is public, so it can be called from outside the module
        pub fn new(radius: f32) -> Circle {
            Circle { radius }
        }

        // Method to check if one circle contains another circle
        // This method is also public
        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

// Test module to verify the functionality of the `Shapes` module
#[cfg(test)]
mod tests {
    // Bring the parent module into scope for testing
    use super::*;

    // Test case to verify that a larger circle contains a smaller circle
    #[test]
    fn larger_circle_should_contains_smaller() {
        // Create a larger circle with a radius of 5.0
        let larger = Shapes::Circle::new(5.0);
        // Create a smaller circle with a radius of 3.0
        let smaller = Shapes::Circle::new(3.0);

        // Assert that the larger circle contains the smaller circle
        assert_eq!(larger.contains(&smaller), true, "Larger Circle not containing smaller");
    }

    // Test case to verify that a smaller circle does not contain a larger circle
    #[test]
    fn smaller_circle_should_not_contain_larger() {
        // Create a larger circle with a radius of 5.0
        let larger = Shapes::Circle::new(5.0);
        // Create a smaller circle with a radius of 3.0
        let smaller = Shapes::Circle::new(3.0);

        // Assert that the smaller circle does not contain the larger circle
        assert_eq!(!smaller.contains(&larger), true);
    }
}
