use chrono::{TimeZone, Utc};
use mongodb::{
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_class])
        .invoke_handler(tauri::generate_handler![create_class])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// Learn mogre about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create_class(name: &str) -> String {
    let classes = client.database("blitz").collection("classes");
    let new_class = doc! {
        "name": format!({name}),
        "blitzes": 0,
        "rounds:" 0,
    };

    let insert_result = classes.insert_one(new_class.clone(), None).await?;
    println!("New Document Id: {}", insert_result.inserted_id)
}

#[tauri::command]
fn save_class(num_of_blitz: u64, rounds_per_blitz: u8) {}
