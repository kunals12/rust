// Definition of a Closure:
// A closure is an anonymous function that can capture variables from its surrounding environment. 
// In Rust, closures are flexible, and their syntax allows for concise function definitions.

struct User {
    name: String,
    age: u8,
    salary: u32,
}

impl User {
    // Constructor method to create a new User instance
    fn new(name: String, age: u8, salary: u32) -> User {
        User { name, age, salary }
    }
}

fn main() {
    // Creating a new User instance with name "Kunal", age 23, and salary 40000
    let person_1 = User::new(String::from("Kunal"), 23, 40000);

    // Defining a closure to validate the user’s name (name should not be empty)
    let validate_user: fn(&str) -> bool = |name: &str| name.len() != 0;

    // Defining a closure to check if the user's age is greater than 20 and salary is above 30000
    let check_age: fn(u8, u32) -> bool = |age: u8, salary: u32| age > 20 && salary > 30000;

    // Printing the user's name and whether the name is valid (true if not empty)
    println!("{}, {}", person_1.name, validate_user(&person_1.name));

    // Checking if the user satisfies the age and salary criteria, and printing the result
    println!(
        "Good to go or not? {}",
        check_age(person_1.age, person_1.salary)
    );
}
