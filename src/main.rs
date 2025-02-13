mod game;
use std::io::{self, Write};
use game::Guess;
use std::process::Command;

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("clear")
            .status()
            .unwrap();
    }
}
fn user_input(msg:&str) -> i32 {
    loop {
        let mut input = String::new();
        print!("{}",msg);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<i32>() {
            Ok(num) => return num, // Return valid number
            Err(_) => println!("Invalid input! Please enter a valid number."),
        }
    }
}


fn main() {
    loop {    
        clear_screen();
        println!("Welcome to the Number Guessing Game!");
        println!("I'm thinking of a number between 1 and 100.");
        println!("Please select the difficulty level:");
        println!("1. Easy (10 chances)");
        println!("2. Medium (5 chances)");
        println!("3. Hard (3 chances)\n\n");


        let level:i32 = user_input("Enter a level: ");
        let mut chances = match level {
            1 => 10,
            2 => 5,
            3 => 3,
            _ => 10,
        };
        let guess = Guess::new(level);
        println!("{}",guess.print_level());

        println!("chances : {}",chances);
        while chances>0 {
            let num = user_input("\n\nEnter your guess: ");
            
            chances -= 1;
            if guess.check(num, &chances){
                break;
            }
        }
        if chances==0{
            println!("\n\nChances left {}.",chances);
        }
        
        let done=user_input("\n\nContinue ? (Yes = 1 and No = any other number): ");
        match done {
            1 => continue,
            _ => break
        }

    }
    
}
