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
    fn main(){
   let num:[i32;9] = [1,2,3,4,5,6,7,8,9];
   for x in 0..9
   { 
      //println!("in position {} there is: {}",x,num[x]);
   }
   //println!("array size is :{}",num.len());
   
   let mut states = [
    "Andhra Pradesh",
    "Arunachal Pradesh",
    "Assam",
    "Bihar",
    "Chhattisgarh",
    "Gujarat",
    "Haryana",
    "Himachal Pradesh",
    "Jammu and Kashmir",
    "Jharkhand",
    "Karnataka",
    "Kerala",
    "Madhya Pradesh",
    "Maharashtra",
    "Manipur",
    "Meghalaya",
    "Mizoram",
    "Nagaland",
    "Odisha",
    "Punjab",
    "Rajasthan",
    "Sikkim",
    "Tamil Nadu",
    "Telangana",
    "Tripura",
    "Uttarakhand",
    "Uttar Pradesh",
    "West Bengal",];
   
   states.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
   
   for i in 0..28
   { 
      println!("state {} there is: {}",i+1,states[i]);
   }
   
}
