*/ use std::io;

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
}*/

use std::io;
fn main() {
       println!("Guess the number!");
      let mut input = String::new();
      println!("Please enter a 1 digit number:");
      io::stdin()
          .read_line(&mut input)
          .expect("Failed to read line");
      println!("You entered: {}", input.trim());
      let number: i32 = 200;
      if input.trim() == "7" {
        println!("You win!");
      }
      else {
        println!("You lose!") 
      }  
}
