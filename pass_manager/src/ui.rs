use std::io::{self, Write};
use crate::Credential;

pub fn menu() {
    println!("\n===== Password Manager =====");
    println!("1. Add Credential");
    println!("2. Search Credential");
    println!("3. List All Credentials");
    println!("4. Delete Credential");
    println!("5. Generate Password");
    println!("q. Exit");
    println!("============================");
    io::stdout().flush().expect("failed to flush the output");
}

pub fn menu_for_search() {
    println!("========== SEARCH MENU ==========");
    println!("1. Search from Website Name");
    println!("2. Search from User Name");
    println!("3. Search from Website and User Name");
    io::stdout().flush().expect("Failed to flush the output");
}

pub fn take_input(prompt: &str) -> String {
    loop {
        print!("{}: ", prompt);
        io::stdout().flush().expect("Failed to flush the output");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");

        let input = input.trim();

        if input.len() == 0 {
            println!("{} cannot be empty. Please try again. \n", prompt);
            continue;
        }

        return input.to_string();
    }
}

pub fn input_id(prompt: &str) -> i64 {
    let id = take_input(prompt);

    match id.parse::<i64>() {
        Ok(num) => num,
        Err(_) => -1
    }
}

pub fn print_credentials(credentials: Vec<Credential>) {
    println!("\n========================== YOUR CREDENTIALS ==========================");
    println!(
        "{:<3}{:<30}{:<20}{:<20}",
        "ID", "WEBSITE NAME", "USER NAME", "PASSWORD"
    );

    if credentials.len() == 0 {
        println!("\t\t\tNO CREDENTIALS SAVED YET");
    }
    
    for cred in credentials {
        println!(
            "{:<3}{:<30}{:<20}{:<20}",
            cred.id, cred.website_name, cred.user_name, cred.password
        );
    }
}

