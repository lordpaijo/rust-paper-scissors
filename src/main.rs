use rand::Rng;
use std::io;
use colored::*;

fn main() {
    println!("{}", "\nWelcome to Rock, Paper, Scissors!\n".yellow());
    let choices = ["rock", "paper", "scissors"];
    let mut user: i32 = 0;
    let mut com: i32 = 0;
    let mut ties: i32 = 0;
    let mut round: u64 = 1;
    loop {
        println!("{}{}", "Round: ".yellow(), round);
        println!("{}\t {}: {}\t {}: {}\t {}: {}", 
            "Enter your choice (rock, paper, scissors):".cyan(), 
            "You".green(), user, "Com".red(), com, "Ties".yellow(), ties);

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read input");
        let user_choice = user_input.trim().to_lowercase();
        
        if !choices.contains(&user_choice.as_str()) && 
          user_choice.as_str().trim().to_lowercase() != "exit" {
            println!("{}","Invalid choice! Please enter rock, paper, or scissors.".red()); 
            return;
        }   if user_choice.as_str().trim().to_lowercase() == "exit"
            { println!("{} {}", "\nExitting game,".red(), "goodbye!".green()); break; }

        let computer_choice = choices[rand::rng().random_range(0..3)];
        match (user_choice.as_str(), computer_choice) {
            ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => { 
                println!("{}", "\nYou win!".green()); 
                println!("{} chose: {}\n", "Computer".red(), computer_choice);
                user += 1; }
            ("rock", "rock") | ("paper", "paper") | ("scissors", "scissors") => { 
                println!("{}", "\nIt's a tie!".yellow()); 
                println!("{} chose: {}\n", "Computer".red(), computer_choice);
                ties += 1; }
            _ => { 
                println!("{}", "\nComputer wins!".red());                         
                println!("{} chose: {}\n", "Computer".red(), computer_choice);
                com += 1 }
        }   round += 1;
    }
}

