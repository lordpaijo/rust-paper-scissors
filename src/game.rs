use rand::Rng;
use std::io;
use colored::*;

#[warn(non_snake_case)]
pub fn run() {
    println!("{}", "\nwelcome to Rust, Paper, Scissors!\n".yellow());
    let choices = ["rock", "paper", "scissors"];
    let mut user: i32 = 0;
    let mut com: i32 = 0;
    let mut ties: i32 = 0;
    let mut rounds: u64 = 1;
    loop {
        header(user, com, ties, rounds);

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read input");
        let user_choice = user_input.trim().to_lowercase();

        if !choices.contains(&user_choice.as_str()) && 
          user_choice.as_str().trim().to_lowercase() != "exit" {
            println!("{}","Invalid choice! Please enter rock, paper, or scissors.".red()); 
            return;
        }   if user_choice.as_str().trim().to_lowercase() == "exit"
            { println!("{} {}", "\nExitting game,".red(), "goodbye!".green()); break; }

        logic(&mut user, &mut com, &mut ties, &mut rounds, &choices, user_choice);
    }
}


fn header(user: i32, com: i32, ties: i32, rounds: u64) {
    println!("{}{}", "round: ".yellow(), rounds);
    println!("{}\t {}: {}\t {}: {}\t {}: {}", 
        "enter your choice (rock, paper, scissors):".cyan(), 
        "you".green(), user, "com".red(), com, "ties".yellow(), ties);
}

fn logic(user:&mut i32, com:&mut i32, ties:&mut i32, rounds:&mut u64, 
  choices: &[&str], user_choice: String) {
    let computer_choice = choices[rand::rng().random_range(0..3)];
    match (user_choice.as_str(), computer_choice) {
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => { 
            println!("{}", "\nYou win!".green()); 
            println!("{} chose: {}\n", "Computer".red(), computer_choice);
            *user += 1; }
        ("rock", "rock") | ("paper", "paper") | ("scissors", "scissors") => { 
            println!("{}", "\nIt's a tie!".yellow()); 
            println!("{} chose: {}\n", "Computer".red(), computer_choice);
            *ties += 1; }
        _ => { 
            println!("{}", "\nComputer wins!".red());                         
            println!("{} chose: {}\n", "Computer".red(), computer_choice);
            *com += 1 }
    }  *rounds += 1;
}
