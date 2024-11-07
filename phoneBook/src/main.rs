struct Contact {
    name: String,
    number: String,
}

impl Contact {
    fn new(name: String, number: String) -> Contact {
        Contact {name, number}
    }
}

enum Actions {
    Add,
    Search,
    ViewAll,
    Quit
}

fn parse_action(input: &str) -> Option<Actions> {
    match input.trim() {
        "add" => Some(Actions::Add),
        "search" => Some(Actions::Search),
        "view" => Some(Actions::ViewAll),
        "quit" => Some(Actions::Quit),
        _ => None
    }
}

fn add_contact(contacts: &mut Vec<Contact>) {
    let mut name: String = String::new();
    let mut number: String = String::new();

    println!("Enter name : ");
    std::io::stdin().read_line(&mut name).expect("Failed to read name");
    println!("Enter number: ");
    std::io::stdin().read_line(&mut number).expect("Failed to read number");

    let contact = Contact::new(name, number);
    contacts.push(contact);

    println!("Contact added successfully!");
}

fn view_all_contacts(contacts: &mut Vec<Contact>) {
    if contacts.is_empty() {
        println!("No contacts available");
    } else {
        println!("contacts:");
        for contact in contacts {
            println!("name: {}  number: {}", contact.name, contact.number);
        }
    }
}

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();
    loop {
        println!("Choose an action : add, search, view, quit");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        match parse_action(&input) {
            Some(Actions::Add) => add_contact(&mut contacts),
            Some(Actions::Search) => println!("Search"),
            Some(Actions::ViewAll) => view_all_contacts(&mut contacts),
            Some(Actions::Quit) => {
                println!("Exiting phone book.");
                break;
            },
            None => println!("Invalid action, try again"),
        }
    }
}
