use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    eprintln!("Guess the number.");

    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The secret numebr is: {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read the line");
        
        // This line will cause the program to crash on invalid input from the user.
        // let guess: u32 = guess.trim().parse().expect("please enter a number.");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess is small."),
            Ordering::Greater => println!("Guess is big."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    // print!("You guessed: {guess}");
    // println!("The secret number is: {secret_number}");
}
