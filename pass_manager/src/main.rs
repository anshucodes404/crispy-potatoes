use std::env;
use std::error::Error;
use sqlx::PgPool;

use crate::repository::*;
use crate::model::*;
use crate::ui::*;

mod model;
mod repository;
mod ui;

async fn add_credential(repo: &CredentialRepository) {
    println!("You chose to add a credential. \n Fill the required details");

    let website_name = take_input("Website Name");
    let user_name = take_input("User Name");
    let password = take_input("Password");

    let credential = NewCredential {
        website_name,
        user_name,
        password,
    };

    match repo.insert(&credential).await {
        Ok(_) => println!("Credential inserted successfully"),
        Err(err) => {
            println!("DB error: {}", err);
        }
    } ;
    
}

async fn search_on_website_name(repo: &CredentialRepository) {
    let website_name = take_input("Website Name");

    match repo
        .find_by_website_name(website_name).await
    {
        Ok(credentials) => {
            print_credentials(credentials);
        }
    
        Err(err) => {
            println!("Database error: {}", err);
        }
    }
}

async fn search_on_user_name(repo: &CredentialRepository) {
    let user_name = take_input("User Name");

    match repo
        .find_by_user_name(user_name).await
    {
        Ok(credentials) => {
            print_credentials(credentials);
        }
    
        Err(err) => {
            println!("Database error: {}", err);
        }
    }
}

async fn search_on_website_name_and_user_name(repo: &CredentialRepository) {
    let website_name = take_input("Website Name");
    let user_name = take_input("User Name");

    match repo
        .find_by_website_name_and_user_name(website_name,user_name,).await
    {
        Ok(credentials) => {
            print_credentials(credentials);
        }
    
        Err(err) => {
            println!("Database error: {}", err);
        }
    }
}

async fn print_all(repo: &CredentialRepository) {
    match repo
        .find_all().await
    {
        Ok(credentials) => {
            print_credentials(credentials);
        }
    
        Err(err) => {
            println!("Database error: {}", err);
        }
    }
}

async fn delete_credential(repo: &CredentialRepository) {
    println!("You chose to delete a credential. \nFill the required details");

    let id = input_id("Enter ID to delete: ");

    match repo.delete(id).await {
        Ok(_) => println!("Credential Deleted successfully"),
        Err(err) => println!("DB error: {}", err)
    }
}



async fn match_choice_for_search(choice: &str, repo: &CredentialRepository) {
    match choice {
        "1" => {
            search_on_website_name(repo).await;
        }

        "2" => {
            search_on_user_name(repo).await;
        }

        "3" => {
            search_on_website_name_and_user_name(repo).await;
        }

        _ => {
            println!("INVALID SEARCH CHOICE");
        }
    }
}

async fn match_the_choice(
    choice: &str,
    repo: &CredentialRepository
) -> Result<bool, Box<dyn Error>> {
    match choice {
        "1" => {
            add_credential(repo).await;
            Ok(false)
        }

        "2" => {
            menu_for_search();
            let search_choice = take_input("Enter choice for Search");
            match_choice_for_search(&search_choice, repo).await;
            Ok(false)
        }

        "3" => {
            print_all(repo).await;
            Ok(false)
        }

        "4" => {
            delete_credential(repo).await;
            Ok(false)
        }

        "q" => {
            Ok(true)
        }

        _ => {
            println!("INVALID CHOICE");
            Ok(false)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().expect("Failed to load env file");

    let local_url = env::var("DATABASE_URL").expect("Database URI not found in .env file");

    let pool = PgPool::connect(&local_url).await?;

    let repo = CredentialRepository::new(pool);

    loop {
        menu();
        let choice = take_input("Enter your choice");
        if match_the_choice(choice.trim(), &repo).await? {
            break;
        }
    }

    Ok(())
}
