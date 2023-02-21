use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;
fn main(){
    println!("{}", "Welcome to the Guessme".blue());

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("{}", secret_number);
    
    loop {
        println!("{}", "Enter your guess: ".blue());

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Not a number");
        let guess :u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Enter a valid number!".red());
                continue;
            },
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "To big".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            },
        }
    }
}
