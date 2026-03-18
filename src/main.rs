fn main() {
    // Introduction line
    println!("Hello, please provide course information below!");

    // Ask for the course name
    println!("Course name: ");

    // Create a mutable variable to save user response
    let mut response1 = String::new()

    // Read user input
    io::stdin()
        .read_line(&mut response1)
        .expect("Failed to read user response.");

    // Ask for the course department
    println!("Course department: ");

}
