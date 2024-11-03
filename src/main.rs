use clap::{Arg, App, SubCommand};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::sync::{Arc, Mutex};

const KV_STORE_FILE: &str = "kv_store.json";
const LIST_STORE_FILE: &str = "list_store.json";

// Struct to represent an in-memory store, now with file persistence
#[derive(Serialize, Deserialize, Default)]
struct InMemoryStore {
    kv_store: HashMap<String, String>,
    list_store: HashMap<String, VecDeque<String>>,
}

impl InMemoryStore {
    fn new() -> Self {
        // Load data from files if they exist; otherwise, return an empty store
        let kv_store = InMemoryStore::load_from_file(KV_STORE_FILE).unwrap_or_default();
        let list_store = InMemoryStore::load_from_file(LIST_STORE_FILE).unwrap_or_default();
        InMemoryStore { kv_store, list_store }
    }

    fn set(&mut self, key: &str, value: &str) {
        self.kv_store.insert(key.to_string(), value.to_string());
        InMemoryStore::save_to_file(KV_STORE_FILE, &self.kv_store).expect("Failed to save KV store");
    }

    fn append(&mut self, list: &str, value: &str) {
        let list_entry = self.list_store.entry(list.to_string()).or_insert_with(VecDeque::new);
        list_entry.push_back(value.to_string());
        InMemoryStore::save_to_file(LIST_STORE_FILE, &self.list_store).expect("Failed to save list store");
    }

    fn display(&self) {
        println!("Key-Value Store:");
        for (key, value) in &self.kv_store {
            println!("  {}: {}", key, value);
        }
        
        println!("\nList Store:");
        for (list, values) in &self.list_store {
            println!("  {}: {:?}", list, values);
        }
    }

    fn save_to_file<T: ?Sized + Serialize>(filename: &str, data: &T) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, data)?;
        Ok(())
    }

    fn load_from_file<T: for<'de> Deserialize<'de> + Default>(filename: &str) -> io::Result<T> {
        if let Ok(file) = File::open(filename) {
            let reader = BufReader::new(file);
            let data = serde_json::from_reader(reader).unwrap_or_default();
            Ok(data)
        } else {
            Ok(Default::default())
        }
    }
}

fn main() {
    let matches = App::new("In-Memory CLI Tool")
        .version("1.0")
        .author("Abhishek Patil")
        .about("CLI tool to interact with an in-memory store")
        .subcommand(
            SubCommand::with_name("set")
                .about("Set a key-value pair")
                .arg(Arg::with_name("key")
                    .help("The key to set")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("value")
                    .help("The value to set")
                    .required(true)
                    .index(2)),
        )
        .subcommand(
            SubCommand::with_name("append")
                .about("Append a value to a list")
                .arg(Arg::with_name("list")
                    .help("The list to append to")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("value")
                    .help("The value to append")
                    .required(true)
                    .index(2)),
        )
        .subcommand(
            SubCommand::with_name("display")
                .about("Display all stored key-value pairs and lists"),
        )
        .get_matches();

    // Create a shared in-memory store using Arc and Mutex for thread-safe access
    let store = Arc::new(Mutex::new(InMemoryStore::new()));

    // Check which command was used
    if let Some(matches) = matches.subcommand_matches("set") {
        let key = matches.value_of("key").unwrap();
        let value = matches.value_of("value").unwrap();
        let mut store = store.lock().unwrap();
        store.set(key, value);
        println!("Set key '{}' to '{}'", key, value);
    } else if let Some(matches) = matches.subcommand_matches("append") {
        let list = matches.value_of("list").unwrap();
        let value = matches.value_of("value").unwrap();
        let mut store = store.lock().unwrap();
        store.append(list, value);
        println!("Appended '{}' to list '{}'", value, list);
    } else if matches.subcommand_matches("display").is_some() {
        let store = store.lock().unwrap();
        store.display();
    } else {
        println!("Invalid command");
        println!("Usage: inmemory-cli [COMMAND]");
        println!("\nCommands:");
        println!("    set <key> <value>     Set a key-value pair");
        println!("    append <list> <value>  Append a value to a list"); 
        println!("    display               Display all stored key-value pairs and lists");
    }
}
