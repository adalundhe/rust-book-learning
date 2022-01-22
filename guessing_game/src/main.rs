use std::io;
use rand::Rng;

fn parse_number(guess: String) -> u16 {
    let guess_number: u16 = guess.trim()
        .parse::<u16>()
        .expect("That's not a number - try again!");

    guess_number
}

fn main() {
    
    println!("Guess the number!");

    println!("Please input your guess...");

    let mut guess = String::new();
    let mut rng = rand::thread_rng();
    let number: u16 = rng.gen_range(1..10);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    let guess_number: u16 = parse_number(guess);
    if number == guess_number {
        println!("Guess matched!");
        println!("You guessed: {} and the system chose: {}", guess_number, number);
    } else {
        println!("You guessed: {} and the system chose: {}", guess_number, number);
    }
    

}
