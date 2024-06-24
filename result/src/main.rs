// Define a struct named `Student` with two fields: `name` and `grade`
// The `grade` field uses the `Option` type to indicate that it might be present (Some) or absent (None)
struct Student {
    name: String,
    grade: Option<u32>,
}

// Define a function `check_student` that takes a student's name and a reference to a vector of `Student` structs
// It returns a `Result` which can be `Ok(Some(grade))` if the student is found and has a grade,
// `Ok(None)` if the student is found but has no grade, or `Err` if the student is not found
fn check_student(student_name: String, student_db: &Vec<Student>) -> Result<Option<u32>, String> {
    // Iterate through each student in the student database
    for student in student_db {
        // If the student's name matches the provided name
        if student_name == student.name {
            // Return the student's grade wrapped in `Ok`
            return Ok(student.grade);
        }
    }
    // If no matching student is found, return `Err` with a message
    Err(String::from("Student Not Found"))
}

fn main() {
    // Create a vector of `Student` structs to represent the student database
    let student_db: Vec<Student> = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(95),
        },
        Student {
            name: String::from("Bob"),
            grade: Some(87),
        },
        Student {
            name: String::from("Charlie"),
            grade: None,
        }
    ];

    // Define a student's name to look up in the database
    let student_name: String = String::from("Bob");

    // Call `check_student` to check if the student is in the database and get the result
    let is_student: Result<Option<u32>, String> = check_student(student_name, &student_db);

    // Use a match expression to handle the `Result`
    match is_student {
        // If the result is `Ok` with some grade
        Ok(grade) => {
            // Use `if let` to check if `grade` is `Some` and print the grade
            if let Some(grade) = grade {
                println!("Student Found and has {grade} grade");
            }
        }
        // If the result is `Err`, print the error message
        Err(err) => println!("{err}")
    }
}
