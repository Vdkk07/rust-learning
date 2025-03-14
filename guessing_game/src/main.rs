use std::{cmp::Ordering, io};
use rand::Rng;

fn main(){
    println!("Guess the number!!");

    let secret_number = rand::rng().random_range(0..=100);

    loop {
            
        println!("Please input your guess:");


        // let secret_number = rand::thread_rng().gen_range(0..=100) // -> thread_rng and gen_range method are deprecated 
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        print!("You gussed: {guess}");

        // Shadowing the guess variable
        let guess:u32 = match guess.trim().parse(){ 
            Ok(num) =>num,
            Err(_) =>{
                println!("Please enter a valid input");
                continue;
            }
        }; 

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too Small")
        }
    }
    }
