use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

// Function to print menu and ask user for input
fn menu() {
    // Get the last scripture read
    let last_scripture = last_entry();

    // Show menu
    println!("Welcome to the Scritpure Tracker");

    // If the last scripture read is not empty, print it
    if last_scripture != "" {
        println!("The last scripture you read was: {}", last_scripture);
    }
    println!("Please select an option");
    println!("1. Add entry");
    println!("2. Show entries");
    println!("3. Exit");
    println!("");
    println!("Enter your choice:");

    // Get input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    // Process input
    if input == "1" {
        add_entry();
    } else if input == "2" {
        show_entries();
    } else if input == "3" {
        println!("Exiting");
        return;
    } else {
        println!("Invalid input");
        menu();
    }
}

// Function to add entry
fn add_entry() {
    println!("Adding entry");
    // Prompt for entry
    println!("Please enter the last scripture you read. (IE Moroni 10:3)");

    // Get input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    // Append input to our scripture file
    append_entry(input);
}

// Function to add the last read scripture to our scripture file
fn append_entry(entry: &str) {
    // Check if the file exists
    if !Path::new("scripture.txt").exists() {
        // If it doesn't, create it
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("scripture.txt")
            .expect("cannot open file");
        // Write to file
        writeln!(file, "{}", entry);

        // Close file
        file.sync_all().expect("Unable to sync file");
    } else {
        // Open file
        let mut file = OpenOptions::new()
            .append(true)
            .open("scripture.txt")
            .expect("cannot open file");

        // Write to file
        writeln!(file, "{}", entry);

        // Close file
        file.sync_all().expect("Unable to sync file");
    }

    // Print success message
    println!("Entry added");

    // Return to menu
    menu();
}

// Function to view the last read scripture entry from our scripture file
fn last_entry() -> String {
    // Open file
    let mut file = match fs::File::open("scripture.txt") {
        Ok(file) => file,
        Err(_err) => {
            //  If file is not found, return an empty string
            return String::new();
        }
    };

    // Read file
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    let last_entry = contents
        .split("\n")
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .last()
        .expect("The file had no parts");

    // Print our last entry
    return last_entry.to_string();
}

// Function to show all the entries in our scripture file
fn show_entries() {
    println!("Here are all your entries:");

    // Open file
    let mut file = match fs::File::open("scripture.txt") {
        Ok(file) => file,
        Err(err) => {
            //  If file is not found, return an empty string
            println!("{}", err);
            return;
        }
    };

    // Read file
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");

    let lines: Vec<String> = file_contents
        .split("\n")
        .map(|s: &str| s.to_string())
        .collect();

    // Print contents
    for line in lines {
        println!("{}", line);
    }
}

fn main() {
    // show menu and do all the work
    menu();
}
