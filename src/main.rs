use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
//---------------------------------------------------------------------------------------
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
//---------------------------------------------------------------------------------------
    loop{
//---------------------------------------------------------------------------------------
        println!("Please input a number: ");
//---------------------------------------------------------------------------------------
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too small!".yellow()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        }
//---------------------------------------------------------------------------------------
    }
//---------------------------------------------------------------------------------------
}   
