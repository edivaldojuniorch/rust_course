use std::io;

fn main() {
    println!("-------------------------------- ");
    println!("----- Guessing a Number Game -----");
    println!("-------------------------------- ");
    println!("I'm thinking of a number between 1 and 100.");
    println!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
    print!("Your guess: {}", guess);
}
