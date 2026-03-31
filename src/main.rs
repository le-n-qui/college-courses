use anyhow::{bail, Result};
use regex::Regex;
use std::io;
use std::str::FromStr;
use std::num::{IntErrorKind, NonZeroU16};

enum Semester {
    Winter,
    Spring,
    Summer,
    Fall
}

impl FromStr for Semester {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {

        match s.to_lowercase().as_str() {
            "wi" | "winter" => Ok(Semester::Winter),
            "sp" | "spr" | "spring" => Ok(Semester::Spring),
            "su" | "summer"  => Ok(Semester::Summer),
            "fa" | "fall"  => Ok(Semester::Fall),
            _ => bail!("Could not determine semester information with given input: {}.", s)
        }

    }
}

fn semester_coding(sem: Semester) -> u8 {
    match sem {
        Semester::Winter => 1,
        Semester::Spring => 2,
        Semester::Summer => 3,
        Semester::Fall => 4
    }
}

fn main() {
    // Introduction line
    println!("Hello, please provide course information below!");

    // Ask for the course name
    println!("1. Course name (e.g. BIOL 50): ");

    // Create a mutable variable to save user response 1
    let mut response1 = String::new();

    // Read user input for question 1
    io::stdin()
        .read_line(&mut response1)
        .expect("Failed to read user response.");

    response1 = response1.trim().to_string();

    // Response 1
    // Request user to enter a string value
    while response1.is_empty() {
        println!("Please input the course prefix and number (e.g. BIOL 50): ");
        
        io::stdin()
            .read_line(&mut response1)
            .expect("Failed to read user input.");

        response1 = response1.trim().to_string();
    }

    // Define the regular expresion
    let reg_ex = Regex::new(r"([[:alpha:]]+)\s*([[:alnum:]]+$)").unwrap();
    let groups = reg_ex.captures(&response1).unwrap();

    // Ask for the course department
    println!("2. Course department: ");

    // Create a mutable variable to save user response 2
    let mut response2 = String::new();

    // Read user input for question 2
    io::stdin()
        .read_line(&mut response2)
        .expect("Failed to read user response.");

    response2 = response2.trim().to_string();

    // Response 2
    // Request user to input a string value
    while response2.is_empty() {
        println!("Please input the course department");

        io::stdin()
            .read_line(&mut response2)
            .expect("Failed to read user input.");

        response2 = response2.trim().to_string();
    }

    // Ask for the course enrollment capacity
    println!("3. Course enrollment capacity: ");

    // Create a mutable variable to save user response 3    
    let mut response3 = String::new();

    // Read user input for question 3
    io::stdin()
        .read_line(&mut response3)
        .expect("Failed to read user response.");

    // Enrollment capacity is a fixed, constant number
    let enroll_cap: NonZeroU16 = match response3.trim().parse() {
        Ok(num) => num,
        Err(error) => match error.kind() {
            IntErrorKind::Empty => {
                println!("No response is inputted."); 
                return;
            },
            IntErrorKind::InvalidDigit => {
                println!("There are invalid characters in the inputted number.");
                return;
            },
            IntErrorKind::PosOverflow => {
                println!("Number is too large to be considered.");
                return;
            },
            IntErrorKind::Zero => {
                println!("Zero cannot be provided.");
                return;
            },
            _ => {
                println!{"Error is encountered."}; 
                return;
            }
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

    while response5.is_empty() {
        println!("Please provide semester in which the course was offered:");

        io::stdin()
            .read_line(&mut response5)
            .expect("Failed to read user response.");

        response5 = response5.trim().to_string();  

    } 

    let semester = match response5.parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Provide known values for semester information. Examples are WI (Winter), SP (Spring), SU, (Summer), FA (Fall).");
            return;
        }
    };

    let sem_code = semester_coding(semester);

    // Show user responses
    println!("\nProvided information: {}, {}, {}, {}, {}, {}", 
        &groups[1], 
        &groups[2],
        response2,
        enroll_cap,
        tot_enroll,
        response5);

    // Show how data is saved
    println!("Data: {}, {}, {}, {}, {}, {}", &groups[1], &groups[2], response2, enroll_cap, tot_enroll, sem_code);
}
