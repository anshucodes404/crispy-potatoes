use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
struct Credential {
    website_name: String,
    user_name: String,
    password: String,
}

const PATH: &str = "/home/codes404/credentials.json";

fn menu() {
    println!("\n===== Password Manager =====");
    println!("1. Add Credential");
    println!("2. Search Credential");
    println!("3. List All Credentials");
    println!("4. Delete Credential");
    println!("5. Generate Password");
    println!("q. Exit");
    println!("============================");
    print!("Enter your choice: ");
    io::stdout().flush().expect("failed to flush the output");
}

fn load_credentials() -> Vec<Credential> {
    let prev_credentials =
        fs::read_to_string(PATH).expect("Not able to read credentials.json file");

    serde_json::from_str(&prev_credentials).expect("Failed to parse json")
}

fn add_credential(credentials: &mut Vec<Credential>) {
    println!("You chose to add a credential. \n Fill the required details");

    let website_name = take_input("Website Name");
    let user_name = take_input("User Name");
    let password = take_input("Password");

    let credential = Credential {
        website_name,
        user_name,
        password,
    };

    if credentials.contains(&credential) {
        println!("Credential already exists");
    } else {
        credentials.push(credential);
        println!("Credential addedd successfully");
    }
}

fn search_on_website_name(credentials: &[Credential]) {
    let website_name = take_input("Website Name");

    print_all_credentials(
        credentials
            .iter()
            .filter(move |cred| cred.website_name == website_name),
    );
}

fn search_on_user_name(credentials: &[Credential]) {
    let user_name = take_input("User Name");

    print_all_credentials(
        credentials
            .iter()
            .filter(move |cred| cred.user_name == user_name),
    );
}

fn search_on_website_name_and_user_name(credentials: &[Credential]) {
    let website_name = take_input("Website Name");
    let user_name = take_input("User Name");

    print_all_credentials(
        credentials
            .iter()
            .filter(move |cred| cred.user_name == user_name && cred.website_name == website_name),
    );
}

fn delete_credential(credentials: &mut Vec<Credential>) {
    println!("You chose to delete a credential. \nFill the required details");

    let website_name = take_input("Website Name");
    let user_name = take_input("User Name");

    credentials.retain(|credential| {
        credential.user_name != user_name || credential.website_name != website_name
    });
}

fn save_file_before_exit(credentials: &[Credential]) -> Result<(), Box<dyn Error>> {
    let file = File::create(PATH)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, credentials)?;

    Ok(())
}

fn take_input(prompt: &str) -> String {
    loop {
        print!("{}: ", prompt);

        let mut input = String::new();

        io::stdout().flush().expect("Failed to flush the output");

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

fn menu_for_search() {
    println!("========== SEARCH MENU ==========");
    println!("1. Search from Website Name");
    println!("2. Search from User Name");
    println!("3. Search from Website and User Name");
}

fn choice_for_search() -> String {

    println!("\nEnter your choice");

    io::stdout().flush().expect("Failed to flush the output");
    
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read the choice");
    choice
}

fn print_all_credentials<'a, I>(credentials: I)
where
    I: IntoIterator<Item = &'a Credential>,
{
    println!("==================== YOUR CREDENTIALS ====================");
    println!(
        "{:<35}{:<20}{:<20}",
        "WEBSITE NAME", "USER NAME", "PASSWORD"
    );
    for cred in credentials {
        println!(
            "{:<35}{:<20}{:<20}",
            cred.website_name, cred.user_name, cred.password
        );
    }
}

fn match_choice_for_search(choice: &str, credentials: &[Credential]) {
    match choice {
        "1" => {
            search_on_website_name(credentials);
        }

        "2" => {
            search_on_user_name(credentials);
        }

        "3" => {
            search_on_website_name_and_user_name(credentials);
        }

        _ => {
            println!("INVALID SEARCH CHOICE");
        }
    }
}

fn match_the_choice(
    choice: &str,
    credentials: &mut Vec<Credential>,
) -> Result<bool, Box<dyn Error>> {
    match choice {
        "1" => {
            add_credential(credentials);
            Ok(false)
        }

        "2" => {
            menu_for_search();
            let search_choice = choice_for_search();
            match_choice_for_search(&search_choice, credentials);
            Ok(false)
        }

        "3" => {
            print_all_credentials(credentials.iter());
            Ok(false)
        }

        "4" => {
            delete_credential(credentials);
            Ok(false)
        }

        "q" => {
            save_file_before_exit(credentials)?;
            Ok(true)
        }

        _ => {
            println!("INVALID CHOICE");
            Ok(false)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut credentials: Vec<Credential> = load_credentials();

    // println!("{:?}", credentials)

    loop {
        menu();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice");

        if match_the_choice(choice.trim(), &mut credentials)? {
            break;
        }
    }

    Ok(())
}
