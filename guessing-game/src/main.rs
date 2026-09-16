use rand::RngExt;
use std::io;

fn main() {
    let secret = rand::rng().random_range(1..=10);
    println!("{secret}");
    loop {
            println!("enter number between 1 and 10");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read line");
            let num: i32 = match input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("input is not a valid interger");
                    continue;
                }
            };
        
            if num == secret {
                println!("correct!");
                return;
            } else {
                println!("incorrect, try again");
            }
    }
    
}
