// Define a struct to store individual contact details
struct Contact {
    name: String,   // Contact's name
    number: String, // Contact's phone number
}

// Implement a constructor for the Contact struct
impl Contact {
    // Creates a new Contact instance with the given name and number
    fn new(name: String, number: String) -> Contact {
        Contact { name, number }
    }
}

// Define an enum to represent different actions the user can take
enum Actions {
    Add,       // Adding a new contact
    Search,    // Searching for an existing contact
    ViewAll,   // Viewing all contacts
    Quit,      // Exiting the application
}

// Function to parse user input into an Actions enum variant
fn parse_action(input: &str) -> Option<Actions> {
    // Match input string to an action; if none match, return None
    match input.trim() {
        "add" => Some(Actions::Add),
        "search" => Some(Actions::Search),
        "view" => Some(Actions::ViewAll),
        "quit" => Some(Actions::Quit),
        _ => None, // If input doesn't match any known action, return None
    }
}

// Function to add a new contact to the phone book
fn add_contact(contacts: &mut Vec<Contact>) {
    let mut name: String = String::new();
    let mut number: String = String::new();

    // Prompt the user for the contact's name
    println!("Enter name: ");
    std::io::stdin().read_line(&mut name).expect("Failed to read name");

    // Prompt the user for the contact's number
    println!("Enter number: ");
    std::io::stdin().read_line(&mut number).expect("Failed to read number");

    // Create a new Contact using trimmed name and number, then add to contacts vector
    let contact = Contact::new(name.trim().to_string(), number.trim().to_string());
    contacts.push(contact);

    println!("Contact added successfully!");
}

// Function to view all contacts in the phone book
fn view_all_contacts(contacts: &Vec<Contact>) {
    if contacts.is_empty() {
        // If no contacts are available, notify the user
        println!("No contacts available");
    } else {
        println!("Contacts:");
        // Loop through all contacts and display each one
        for contact in contacts {
            println!("Name: {}  Number: {}", contact.name, contact.number);
        }
    }
}

// Function to search for a contact by name
fn search_contact(contacts: &Vec<Contact>) {
    let mut name = String::new();
    println!("Please enter name to search contact: ");
    std::io::stdin().read_line(&mut name).expect("Failed to read contact name");

    // Trim whitespace and convert input to lowercase for a case-insensitive search
    let name = name.trim().to_lowercase();

    // Iterate over the contacts vector to find a matching contact
    for contact in contacts.iter() {
        if contact.name.to_lowercase() == name {
            // If a matching contact is found, display its details
            println!("Found contact: {} - {}", contact.name, contact.number);
            return; // Exit function after finding the contact
        }
    }
    // If no contact is found, notify the user
    println!("Contact not found!");
}

// Main function to run the program
fn main() {
    // Initialize an empty vector to store contacts
    let mut contacts: Vec<Contact> = Vec::new();
    loop {
        // Display available actions to the user
        println!("Choose an action: add, search, view, quit");

        // Read user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        // Parse the input into an action and execute the corresponding function
        match parse_action(&input) {
            Some(Actions::Add) => add_contact(&mut contacts),   // Add a new contact
            Some(Actions::Search) => search_contact(&contacts), // Search for a contact
            Some(Actions::ViewAll) => view_all_contacts(&contacts), // View all contacts
            Some(Actions::Quit) => {
                // Exit the application if the user chooses "quit"
                println!("Exiting phone book.");
                break;
            },
            None => println!("Invalid action, try again"), // Handle unrecognized input
        }
    }
}
