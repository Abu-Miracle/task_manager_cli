use serde::{self, Serialize, de::DeserializeOwned};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::{error::Error, fmt::Display};

use crate::Task;

const TASKS_FILE: &str = "src/task_manager.json";

pub fn save_tasks<'a, T>(tasks: &[Task<T>]) -> Result<(), String>
where
    T: Display + Serialize,
{
    // Convert Task to JSON format
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|e| format!("Failed to serialize tasks: {}", e))?;

    // Open/Create the file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(TASKS_FILE)
        .map_err(|e| format!("Failed to open file: {}", e))?;

    // Write tasks (as JSON) to the file
    file.write_all(json.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(())
}

pub fn load_tasks<T>() -> Result<Vec<Task<T>>, String>
where
    T: DeserializeOwned + Display,
{
    // Checks if the file exists
    if !Path::new(TASKS_FILE).exists() {
        return Ok(Vec::new()); // returns an empty vector of no FIle
    }

    // Open the file
    let mut file = File::open(TASKS_FILE).map_err(|e| format!("Failed to open file: {}", e))?;

    // Read the file content into string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Handle empty file
    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }

    // Parse content back to Vec<Task<T>>
    let tasks: Vec<Task<T>> =
        serde_json::from_str(&contents).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(tasks)
}
