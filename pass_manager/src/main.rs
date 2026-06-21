use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::BufWriter;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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
}

fn load_credentials() -> Vec<Credential> {
    let prev_credentials =
        fs::read_to_string(PATH).expect("Not able to read credentials.json file");

    serde_json::from_str(&prev_credentials).expect("Failed to parse json")
}

fn save_file_before_exit(credentials: &[Credential]) -> Result<(), Box<dyn Error>> {
    let file = File::create(PATH)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, credentials)?;

    Ok(())
}

fn add_credential(credential: Credential, credentials: &mut Vec<Credential>) {
    if credentials.contains(&credential) {
        println!("Credential already exists");
    } else {
        credentials.push(credential);
        println!("Credential addedd successfully");
    }
}

fn search_on_website_name<'a>(credentials: &'a [Credential], website_name: &str) -> Vec<&'a Credential> {
    credentials
        .iter()
        .filter(|cred| cred.website_name == website_name)
        .collect()
}

fn search_on_user_name<'a>(credentials: &'a [Credential], user_name: &str) -> Vec<&'a Credential> {
    credentials
        .iter()
        .filter(|cred| cred.user_name == user_name)
        .collect()
}

fn search_on_website_name_and_user_name<'a>(credentials: &'a [Credential], user_name: &str, website_name: &str) -> Vec<&'a Credential> {
    credentials
        .iter()
        .filter(|cred| cred.user_name == user_name && cred.website_name == website_name)
        .collect()
}

fn delete_credential(credentials: &mut Vec<Credential>, user_name: &str, website_name: &str) {
    credentials
        .retain(|credential| credential.user_name != user_name || credential.website_name != website_name);
}

fn main() {
    let credentials: Vec<Credential> = load_credentials();

    // println!("{:?}", credentials)

    loop {
        menu();
    }
}
