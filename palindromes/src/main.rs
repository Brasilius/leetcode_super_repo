
fn main() {
    let mut palindrome:i32;
    println!("Starting loop...");
    loop { 
        println!("loop built!");
        println!("Do you want to continue? (yes/no)");
        // Get user input
         let mut user_input = String::new();
         std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    
        // Check if the user wants to quit
        match user_input.trim() {
            "yes" => {
                println!("please enter a number to check for palindromeness: ");
                let mut user_input = String::new();
                std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
                let palindrome = user_input.parse::<i32>();
                let palindrome = palindrome.unwrap();
                palindrome_decoder(palindrome);
                break},
            "no" => continue,
            _ => println!("I didn't understand that. Please answer 'yes' or 'no'."),
           }
     }
}
    
fn palindrome_decoder(palindrome:i32){
let palindrome_to_string: String = palindrome.to_string();
let reversed_palindrome: String = palindrome_to_string.chars().rev().collect();
if palindrome_to_string == reversed_palindrome{
    println!("palindrome moment");
}
else {
    println!("not a palindrome");
}
}
