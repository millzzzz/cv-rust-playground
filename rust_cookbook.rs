// Rust Cookbook: Practical Recipes for Common Tasks
// This file contains practical examples of common programming tasks in Rust

// ===== STRING MANIPULATION =====

/// Convert a string to uppercase
fn recipe_uppercase_string() {
    let text = "hello, world";
    let uppercase = text.to_uppercase();
    println!("{}", uppercase); // Prints: HELLO, WORLD
    
    // For owned String
    let mut owned_string = String::from("hello");
    owned_string.make_ascii_uppercase();
    println!("{}", owned_string); // Prints: HELLO
}

/// Split a string by delimiter
fn recipe_split_string() {
    let text = "apple,banana,cherry";
    
    // Split by comma
    let fruits: Vec<&str> = text.split(',').collect();
    println!("{:?}", fruits); // Prints: ["apple", "banana", "cherry"]
    
    // Iterate over splits without collecting
    for fruit in text.split(',') {
        println!("Fruit: {}", fruit);
    }
}

/// Join strings together
fn recipe_join_strings() {
    let fruits = vec!["apple", "banana", "cherry"];
    
    // Join with a delimiter
    let text = fruits.join(", ");
    println!("{}", text); // Prints: apple, banana, cherry
    
    // Using string format
    let formatted = format!("{} and {}", "Rust", "Cooking");
    println!("{}", formatted); // Prints: Rust and Cooking
}

// ===== FILE OPERATIONS =====

/// Read a file to string
fn recipe_read_file() -> std::io::Result<()> {
    use std::fs;
    
    // Read entire file to string
    let contents = fs::read_to_string("filename.txt")?;
    println!("File contents: {}", contents);
    
    Ok(())
}

/// Write string to file
fn recipe_write_file() -> std::io::Result<()> {
    use std::fs;
    
    let data = "Hello, Rust Cookbook!";
    fs::write("output.txt", data)?;
    println!("Data written to file");
    
    Ok(())
}

/// Read file line by line
fn recipe_read_lines() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    
    let path = Path::new("filename.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    for line in reader.lines() {
        println!("{}", line?);
    }
    
    Ok(())
}

// ===== ERROR HANDLING =====

/// Using Result for functions that can fail
fn recipe_result_handling() -> Result<(), std::io::Error> {
    let file_result = std::fs::File::open("nonexistent.txt");
    
    // Method 1: Match expression
    match file_result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Error opening file: {}", error),
    }
    
    // Method 2: Using ? operator (propagates error)
    let _file = std::fs::File::open("another_file.txt")?;
    
    // Method 3: unwrap_or_else with custom error handling
    let _file = std::fs::File::open("third_file.txt").unwrap_or_else(|error| {
        panic!("Failed to open file: {}", error);
    });
    
    Ok(())
}

/// Using Option for values that might be absent
fn recipe_option_handling() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Find first even number
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    
    // Method 1: Match expression
    match first_even {
        Some(number) => println!("First even number: {}", number),
        None => println!("No even numbers found"),
    }
    
    // Method 2: if let for concise matching
    if let Some(number) = first_even {
        println!("Found even number: {}", number);
    }
    
    // Method 3: unwrap_or for default values
    let value = first_even.unwrap_or(&0);
    println!("First even number or default: {}", value);
}

// ===== COLLECTIONS =====

/// Working with vectors
fn recipe_vectors() {
    // Create a new vector
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // Add elements
    numbers.push(6);
    
    // Access elements
    println!("Third element: {}", numbers[2]);
    
    // Safe access with get (returns Option)
    match numbers.get(10) {
        Some(value) => println!("Value at index 10: {}", value),
        None => println!("No value at index 10"),
    }
    
    // Iterate over vector
    for number in &numbers {
        println!("{}", number);
    }
    
    // Modify elements while iterating
    for number in &mut numbers {
        *number *= 2;
    }
    
    println!("Modified vector: {:?}", numbers);
}

/// Working with HashMaps
fn recipe_hashmaps() {
    use std::collections::HashMap;
    
    // Create a new HashMap
    let mut scores = HashMap::new();
    
    // Insert key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    
    // Access values
    match scores.get("Blue") {
        Some(score) => println!("Blue team score: {}", score),
        None => println!("Blue team not found"),
    }
    
    // Insert only if key doesn't exist
    scores.entry(String::from("Yellow")).or_insert(25);
    
    // Update value based on old value
    let blue_score = scores.entry(String::from("Blue")).or_insert(0);
    *blue_score += 5;
    
    println!("Scores: {:?}", scores);
}

// ===== CONCURRENCY =====

/// Spawn a thread
fn recipe_basic_threading() {
    use std::thread;
    use std::time::Duration;
    
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread: number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // Main thread code
    for i in 1..5 {
        println!("Main: number {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Wait for spawned thread to finish
    handle.join().unwrap();
}

/// Share data between threads using channels
fn recipe_channels() {
    use std::sync::mpsc;
    use std::thread;
    
    // Create a channel
    let (tx, rx) = mpsc::channel();
    
    // Spawn thread that sends messages
    thread::spawn(move || {
        let messages = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    
    // Receive messages
    for received in rx {
        println!("Got: {}", received);
    }
}

// ===== COMMAND LINE ARGS =====

/// Parse command line arguments
fn recipe_command_args() {
    // Simple argument access
    let args: Vec<String> = std::env::args().collect();
    println!("Program arguments: {:?}", args);
    
    // Note: In a real application, you would typically use a crate like clap
    // for more robust command-line argument parsing
}

// Main function to demonstrate usage
fn main() {
    println!("Rust Cookbook - Practical Recipes");
    println!("Uncomment functions below to run specific recipes");
    
    // String manipulation
    // recipe_uppercase_string();
    // recipe_split_string();
    // recipe_join_strings();
    
    // File operations
    // if let Err(e) = recipe_read_file() {
    //     eprintln!("Error: {}", e);
    // }
    
    // Collections
    // recipe_vectors();
    // recipe_hashmaps();
    
    // Concurrency
    // recipe_basic_threading();
    // recipe_channels();
    
    println!("See code comments for more recipes and explanations");
} 