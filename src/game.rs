use rand::Rng;
use std::io;
use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::stdout;
use colored::*;

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 { return false; }
    }
    true
}

pub fn run(
    mut current_round: u64,rounds_limit: Option<u64>,
    auto: bool, skip: Option<u64>, skip_rounds: Option<Vec<u64>>, 
    skip_even: bool, skip_odd: bool, skip_prime: bool, 
    set_even: bool, set_odd: bool, set_prime: bool
) -> bool {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    println!("{}{}, {}, {}!\n", "\nWelcome to ".yellow().bold(), 
        "Rust".truecolor(214, 93, 14).bold(), "Paper".green().bold(), "Scissors".blue().bold());

    let choices = ["rock", "paper", "scissors"];
    let mut user: i32 = 0;
    let mut com: i32 = 0;
    let mut ties: i32 = 0;

    loop {
        if let Some(limit) = rounds_limit 
        { if current_round > limit {
            println!("{}","Round limit reached!".blue().bold());
            print_final_score(user, com, ties); return false; }
        }
        if skip == Some(current_round) 
            || (skip_rounds.as_ref().map_or(false, |rounds| rounds.contains(&current_round)))
            || (skip_even && current_round % 2 == 0)
            || (skip_odd && current_round % 2 != 0)
            || (skip_prime && is_prime(current_round)) 
        { println!("Skipping round {}\n", current_round); current_round += 1; continue; }
        if (set_even && current_round % 2 != 0)
            || (set_odd && current_round % 2 == 0)
            || (set_prime && !is_prime(current_round)) 
        { current_round += 1; continue; }

        header(user, com, ties, current_round);

        let user_choice = if auto {
            let random_choice = choices[rand::rng().random_range(0..3)];
            println!("{} {}", "(Auto) You chose:".cyan().bold(), random_choice);
            random_choice.to_string()
        } else {
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read input");
            user_input.trim().to_lowercase()
        };

        let result = logic(&mut user, &mut com, &mut ties, &mut current_round, &choices, user_choice);
        if result == Some(false) {
            return false;
        } else if result == Some(true) {
            return true;
        }
    }
}


fn header(user: i32, com: i32, ties: i32, rounds: u64) {
    println!("{}{}", "round: ".yellow(), rounds);
    println!("{} \t{}: {} \t{}: {} \t{}: {}", 
        "enter your choice (rock, paper, scissors):".cyan(), 
        "you".green(), user, "com".red(), com, "ties".yellow(), ties);
}

fn logic(user:&mut i32, com:&mut i32, ties:&mut i32, rounds:&mut u64, 
  choices: &[&str], user_choice: String) -> Option<bool> {
    if !choices.contains(&user_choice.as_str()) && 
        (user_choice.as_str().trim().to_lowercase() == "exit" ||
         user_choice.as_str().trim().to_lowercase() == "quit")
            { println!("{} {}", "\nQuitting game,".red(), "goodbye!".green()); return Some(false); }
    if !choices.contains(&user_choice.as_str()) && 
        (user_choice.as_str().trim().to_lowercase() == "restart" ||
         user_choice.as_str().trim().to_lowercase() == "reset") 
            { println!("{}", "\nRestarting your game...".yellow().bold()); 
                std::thread::sleep(std::time::Duration::from_secs(1)); return Some(true); }
    else if !choices.contains(&user_choice.as_str()) && 
        user_choice.as_str().trim().to_lowercase() == "end"
            { println!("\n{}\n\n{}: {} \t{}: {} \t{}: {}", "Ending game...".blue().bold(),
                "Wins".green().bold(), *user, "Loses".red().bold(), *com, "Ties".yellow().bold(), *ties); 
            if *user > *com { 
                println!("\n{} {} {}", "You have more".yellow().bold(), "Wins".green().bold(),
                "than the computer, nice!".yellow().bold()); }
            else if *user < *com { 
                println!("\n{} {} {}", "You have more".yellow().bold(), "Loses".red().bold(),
                "than the computer, nice try...".yellow().bold()); }
            else { println!("{}", "Well done, you both hit a tie!".yellow().bold()); }
            return Some(false); } 
    else if !choices.contains(&user_choice.as_str()) {
        println!("{}","\nInvalid choice! Please enter rock, paper, or scissors.\n".red()); 
        return None; }

    let computer_choice = choices[rand::rng().random_range(0..3)];
    match (user_choice.as_str(), computer_choice) {
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => { 
            println!("{}", "\nYou win!".green().bold()); 
            println!("{} chose: {}\n", "Computer".red(), computer_choice);
            *user += 1; }
        ("rock", "rock") | ("paper", "paper") | ("scissors", "scissors") => { 
            println!("{}", "\nIt's a tie!".yellow().bold()); 
            println!("{} chose: {}\n", "Computer".red(), computer_choice);
            *ties += 1; }
        _ => { 
            println!("{}", "\nComputer wins!".red().bold());                         
            println!("{} chose: {}\n", "Computer".red(), computer_choice);
            *com += 1 }
    }  *rounds += 1;
    None
}

fn print_final_score(user: i32, com: i32, ties: i32) {
    println!("\n{}: {} \t{}: {} \t{}: {}", "Wins".green().bold(), user, 
            "Loses".red().bold(), com, "Ties".yellow().bold(), ties);
    if user > com { println!("\n{} {} {}", "You have more".yellow().bold(), "Wins".green().bold(), 
            "than the computer, nice!".yellow().bold()); }
    else if user < com { println!("\n{} {} {}", "You have more".yellow().bold(), "Loses".red().bold(), 
            "than the computer, nice try...".yellow().bold()); }
    else { println!("{}", "Well done, you both hit a tie!".yellow().bold()); }
}
