use anyhow::{bail, Result};
use regex::Regex;
use std::io;
use std::str::FromStr;
use std::num::NonZeroU16;

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

fn number_checking(datapoint: &str) -> i16 {
    let res = match datapoint.parse::<NonZeroU16>() {
        Ok(num) => num.get() as i16,
        Err(_) => -1
    };

    res
}

fn main() {
    // Introduction line
    println!("Hello, please provide course information below!");

    /////////////
    // Question 1
    /////////////

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


    /////////////
    // Question 2
    /////////////
    
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


    /////////////
    // Question 3
    /////////////

    // Ask for the course enrollment capacity
    println!("3. Course enrollment capacity: ");

    // Create a mutable variable to save user response 3    
    let mut response3 = String::new();

    // Read user input for question 3
    io::stdin()
        .read_line(&mut response3)
        .expect("Failed to read user response.");

    // Enrollment capacity is a fixed, constant number
    // When -1 is returned, we encounter errors
    // It indicates that further data checking is 
    // needed to be done in person
    let enroll_cap = number_checking(&response3.trim());


    /////////////
    // Question 4
    /////////////

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


    /////////////
    // Question 5
    /////////////

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

    /////////////
    // Question 6
    /////////////

    // Display question 6, asking for user response
    println!("6. Year:");

    let mut response6 = String::new();

    // Read in user response
    io::stdin()
        .read_line(&mut response6)
        .expect("Failed to read user response.");

    // Continue to request for response if no response is provided    
    while response6.trim().is_empty() {
        println!("Please provide information on the year in which the course was provided.");

        io::stdin()
            .read_line(&mut response6)
            .expect("Failed to read user response.");
    }
   
    // When -1 is returned, we encounter errors
    // It indicates that further data checking is 
    // needed to be done in person
    let year = number_checking(&response6.trim());


    // Show user responses
    println!("\nProvided information: {}, {}, {}, {}, {}, {}, {}", 
        &groups[1], 
        &groups[2],
        response2,
        enroll_cap,
        tot_enroll,
        response5, 
        year);

    // Show how data is saved
    println!("Data: {}, {}, {}, {}, {}, {}, {}", &groups[1], &groups[2], response2, enroll_cap, tot_enroll, sem_code, year);
}
