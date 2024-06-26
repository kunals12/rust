mod Shapes {
    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Circle {
            Circle { radius }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_circle_should_contains_smaller() {
        let larger = Shapes::Circle::new(5.0);
        let smaller = Shapes::Circle::new(3.0);

        assert_eq!(larger.contains(&smaller), true, "Larger Circle not containing smaller");
    }
    #[test]
    fn smaller_circle_should_not_contain_larger() {
        let larger = Shapes::Circle::new(5.0);
        let smaller = Shapes::Circle::new(3.0);

        assert_eq!(smaller.contains(&larger), false);
    }
}