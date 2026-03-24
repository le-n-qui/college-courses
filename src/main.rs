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

    response1 = response1.trim().to_string();

    // Ask for the course department
    println!("2. Course department: ");

    // Create a mutable variable to save user response 2
    let mut response2 = String::new();

    // Read user input for question 2
    io::stdin()
        .read_line(&mut response2)
        .expect("Failed to read user response.");

    response2 = response2.trim().to_string();

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

    // Ask for the course total enrollment after enrollment census day
    println!("4. Course total enrollment: ");

    // Create a mutable variable to save user response 4
    let mut response4 = String::new();

    // Read in user input for question 4
    io::stdin()
        .read_line(&mut response4)
        .expect("Failed to read user response.");

    // Total enrollment is a fixed number, recorded after enrollment census day
    let tot_enroll: u16 = match response4.trim().parse() {
        Ok(num) => num,
        Err(error) => {
            println!("Please input a number. Warning: {}", error);
            return;
        }
    };

    // Ask for the semester that the course is taught in
    println!("5. Semester: ");

    // Create a mutable variable to save user response 5
    let mut response5 = String::new();

    // Read in user input for question 5
    io::stdin()
        .read_line(&mut response5)
        .expect("Failed to read user response.");

    response5 = response5.trim().to_string();

    // Semester is a string
    let semester: String;

    if response5.is_empty() {
        semester = String::from("No response");
        println!("Please provide semester information.");
    } 
    else {
        semester = response5.trim().to_string();
    }
    

    // Show user responses
    println!("\nProvided information: {0}, {1}, {2}, {3}, {4}", response1, response2, enroll_cap, tot_enroll, semester);
}
