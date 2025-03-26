use std::io;

fn main() {
    // Create a new String to store user input
    let mut input = String::new();
    
    // Prompt the user
    println!("Please enter some text:");
    
    // Read user input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    // Trim and print the input
    println!("You entered: {}", input.trim());
}
