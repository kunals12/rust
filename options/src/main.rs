// Define a struct named `Student` with two fields: `name` and `grade`
// The `grade` field uses the `Option` type to indicate that it might be present (Some) or absent (None)
struct Student {
    name: String,
    grade: Option<u32>,
}

// Define a function `get_grade` that takes a reference to a student's name and a reference to a vector of `Student` structs
// It returns an `Option<u32>` which will be `Some(grade)` if the student is found, or `None` if not found
fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    // Iterate through each student in the student database
    for student in student_db {
        // If the student's name matches the provided name
        if student.name == *student_name {
            // Return the student's grade (which is an Option<u32>)
            return student.grade;
        }
    }
    // If no matching student is found, return `None`
    None
}

fn main() {
    // Create a vector of `Student` structs to represent the student database
    let student_db = vec![
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
    let student_name = String::from("Bob");
    // Get the grade of the student by calling `get_grade`
    let student_grade = get_grade(&student_name, &student_db);

    // Use `if let` to check if `student_grade` is `Some(grade)`
    if let Some(grade) = student_grade {
        // If `Some(grade)`, print the grade
        println!("grade is: {grade}");
    }
}
