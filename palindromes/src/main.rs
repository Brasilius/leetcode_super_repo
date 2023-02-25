
fn main() {
    let mut palindrome = 1;
    println!("Starting loop...");
    loop { 
        println!("loop built!");
        println!("Do you want to continue? (yes/no)");
        // Get user input
         let mut user_input = String::new();
         std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    
        // Check if the user wants to quit
        match user_input.trim() {
            "yes" => break,
            "no" => continue,
            _ => println!("I didn't understand that. Please answer 'yes' or 'no'."),
           }
     }
}
    
fn pub palindrome_decoder(){
let mut palindrome_forward_ways_test = String::new(palindrome);



}
