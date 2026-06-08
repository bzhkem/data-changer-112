use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use serde_json::Value;
use std::env;

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("=== 112 Operator Save Editor ===\n");

    loop {
        println!("Choose save location:");
        println!("  1) Use default location (AppData\\LocalLow\\JutsuGames\\112 Operator\\Saves)");
        println!("  2) Enter custom path");
        println!("  0) Exit");

        let choice = read_input("\nYour choice: ");

        let save_dir: PathBuf = match choice.as_str() {
            "0" => {
                println!("Exiting editor.");
                return;
            }
            "1" => {
                let user = match env::var("USERNAME") {
                    Ok(u) => u,
                    Err(_) => {
                        println!("Failed to detect user automatically. Returning to menu.\n");
                        continue;
                    }
                };
                let mut path = PathBuf::from("C:\\Users");
                path.push(user);
                path.push("AppData\\LocalLow\\JutsuGames\\112 Operator\\Saves");
                if !path.exists() || !path.is_dir() {
                    println!("Default save directory not found. Returning to menu.\n");
                    continue;
                }
                path
            }
            "2" => {
                let custom = read_input("Enter the path to your save directory: ");
                let path = PathBuf::from(custom);
                if !path.exists() || !path.is_dir() {
                    println!("Directory not found. Returning to menu.\n");
                    continue;
                }
                path
            }
            _ => {
                println!("Invalid choice. Try again.\n");
                continue;
            }
        };
        
        let filename = read_input("Enter the save file name (e.g., Save_free_game_autosave_129_EASY.json): ");
        let mut save_path = save_dir.clone();
        save_path.push(&filename);

        if !save_path.exists() || !save_path.is_file() {
            println!("File not found at {:?}. Returning to start.\n", save_path);
            continue;
        }
        let file_data = match fs::read_to_string(&save_path) {
            Ok(data) => data,
            Err(e) => {
                println!("Failed to read file: {}. Returning to start.\n", e);
                continue;
            }
        };

        let mut json: Value = match serde_json::from_str(&file_data) {
            Ok(j) => j,
            Err(e) => {
                println!("Invalid JSON: {}. Returning to start.\n", e);
                continue;
            }
        };

        loop {
            let keys: Vec<String> = match json.as_object() {
                Some(o) => o.keys().cloned().collect(),
                None => {
                    println!("JSON root is not an object. Returning to start.\n");
                    break;
                }
            };

            if keys.is_empty() {
                println!("No editable fields found. Returning to start.\n");
                break;
            }
            println!("\nAvailable fields:\n");
            for (i, key) in keys.iter().take(80).enumerate() {
                println!("{:>3}) {}", i + 1, key);
            }
            println!("  0) Exit");

            let choice = read_input("\nSelect a number to edit: ");
            if choice == "0" {
                println!("Returning to main menu.\n");
                break;
            }

            let index: usize = match choice.parse::<usize>() {
                Ok(n) if n > 0 && n <= keys.len() => n - 1,
                _ => {
                    println!("Invalid selection. Try again.\n");
                    continue;
                }
            };

            let selected_key = &keys[index];

            if let Some(obj) = json.as_object_mut() {
                let current_value = &obj[selected_key];
                println!("\nSelected: {}\nCurrent value: {}", selected_key, current_value);

                match current_value {
                    Value::Bool(b) => {
                        let new_value = read_input("Toggle value? (y/n): ");
                        match new_value.to_lowercase().as_str() {
                            "y" | "yes" => { obj.insert(selected_key.clone(), Value::Bool(!b)); },
                            "n" | "no" => {},
                            _ => {
                                println!("Invalid input, must be y or n. Returning to menu.\n");
                                continue;
                            }
                        }
                    }
                    Value::Number(_) => {
                        let new_value = read_input("Enter new value (number): ");
                        let parsed_value: i64 = match new_value.parse() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("Only numeric values supported. Returning to menu.\n");
                                continue;
                            }
                        };
                        obj.insert(selected_key.clone(), Value::from(parsed_value));
                    }
                    _ => {
                        println!("Editing this type is not supported. Returning to menu.\n");
                        continue;
                    }
                }
            }

            // Save JSON file
            match fs::write(&save_path, serde_json::to_string_pretty(&json).unwrap()) {
                Ok(_) => println!("\n'{}' updated successfully!\n", selected_key),
                Err(e) => println!("Failed to write file: {}. Returning to menu.\n", e),
            }
        }
    }
}
