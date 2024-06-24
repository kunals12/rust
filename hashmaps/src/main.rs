use std::collections::HashMap;

fn main() {
    // Create a HashMap to store names and ages
    let mut person: HashMap<&str, u32> = HashMap::new();
    
    // Insert some key-value pairs into the HashMap
    person.insert("Nouman", 32);
    person.insert("Kunal", 23);
    
    // Print the age of the person with name "Kunal"
    println!("The age is {:?}", person.get("Kunal").unwrap());

    // Check if the key "Nouman" exists in the HashMap
    if person.contains_key("Nouman") {
        println!("Key exists");
    } else {
        println!("Key doesn't exist");
    }

    // Use match to check if the key "Kunal" exists and print appropriate message
    match person.get("Kunal") {
        Some(_) => println!("Value exists"),
        None => println!("Not exists"),
    }

    // Iterate over the key-value pairs in the HashMap
    // The correct types should be &str and u32 because we are borrowing the values from the HashMap
    for (name, age) in &person {
        println!("The person {} has an age of {}", name, age);
    }
}
