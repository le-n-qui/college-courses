use std::io;

fn main() {
    // Introduction line
    println!("Hello, please provide course information below!");

    // Ask for the course name
    println!("1. Course name: ");

    // Create a mutable variable to save user response 1
    let mut response1 = String::new();

    // Read user input for question 1
    io::stdin()
        .read_line(&mut response1)
        .expect("Failed to read user response.");

    // Ask for the course department
    println!("2. Course department: ");

    // Create a mutable variable to save user response 2
    let mut response2 = String::new();

    // Read user input for question 2
    io::stdin()
        .read_line(&mut response2)
        .expect("Failed to read user response.");

    // Ask for the course enrollment capacity
    println!("3. Course enrollment capacity: ");

    // Create a mutable variable to save user response 3    
    let mut response3 = String::new();

    // Read user input for question 3
    io::stdin()
        .read_line(&mut response3)
        .expect("Failed to read user response.");

    // Enrollment capacity is a fixed, constant number
    let enroll_cap: u16 = match response3.trim().parse() {
        Ok(num) => num,
        Err(error) => {
            println!("Please input a number. Warning: {}", error);
            return;
        }
    };

    // Show user responses
    println!("\nProvided information: {0}, {1}, {2}", response1, response2, enroll_cap);
}
