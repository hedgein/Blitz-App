use chrono::{TimeZone, Utc};
use mongodb::{
    bson::{doc, to_document, Document},
    sync::{Client, Collection},
};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use tauri::Manager;
use tokio;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Class {
    name: String,
    blitzes: u64,
    rounds: u8,
}

#[derive(Clone, Serialize)]
struct Payload {
    message: String,
}
fn main() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .setup(|app| {
            // Load the MongoDB connection string from an environment variable:
            let client_uri =
                env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

            // A Client is needed to connect to MongoDB:
            // An extra line of code to work around a DNS issue on Windows:
            let client = mongodb::sync::Client::with_uri_str(client_uri)?;
            // Print the databases in our MongoDB cluster:
            for name in client.list_database_names(None, None)? {
                println!("- {}", name);
            }

            let classes: Collection<Document> = client.database("blitz").collection("classes");
            // listen to the `event-name` (emitted on any window)
            let create_class = app.listen_global("create-class", move |event| {
                let name = event
                    .payload()
                    .unwrap()
                    .to_string()
                    .replace(&['\"', '\''][..], "");
                let new_class: Class = Class {
                    name: name,
                    blitzes: 0,
                    rounds: 0,
                };
                println!("New Class: {:?}", new_class);
                let bson_doc = to_document(&new_class);
                let insert_result = classes.insert_one(bson_doc.unwrap(), None);
                println!("New Document Id: {}", insert_result.unwrap().inserted_id);
            });

            // let add_class = app.listen_global("add-class", move |event| {
            //     let new_class: Class = Class {
            //         name: event.payload().unwrap().to_string(),
            //         blitzes: 0,
            //         rounds: 0,
            //     };
            //     println!("New Class: {:?}", new_class);
            //     let bson_doc = to_document(&new_class);
            //     let insert_result = classes.insert_one(bson_doc.unwrap(), None);
            //     println!("New Document Id: {}", insert_result.unwrap().inserted_id);
            // });
            // unlisten to the event using the `id` returned on the `listen_global` functionx
            // an `once_global` API is also exposed on the `App` struct
            // app.unlisten(id);

            // emit the `event-name` event to all webview windows on the frontend
            // app.emit_all(
            //     "saveClassName",
            //     Payload {
            //         message: "Tauri is awesome!".into(),
            //     },
            // )
            // .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_class])
        // .invoke_handler(tauri::generate_handler![create_class])
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
fn save_class(num_of_blitz: u64, rounds_per_blitz: u8) {}
