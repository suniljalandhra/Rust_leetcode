use rand::Rng;
use colored::*;
use std::io;
fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Your random number : {}",secret_number);
    loop {
        println!("please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line( &mut guess)
            .expect("failed to readline");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed : {}",guess);
    
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{}","too small".red()),
            std::cmp::Ordering::Equal => {
                println!("{}","you win".green());
                break;
            }, 
            std::cmp::Ordering::Greater => println!("{}","too big".red()),
        }
    }
  



}
