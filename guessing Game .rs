use rand::*;
use std::*;
fn main() {
    println!("Guess the number!");

    let mut guess = String::new();

    let number = rand::thread_rng().gen_range(1, 2);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // input 4 read as 4\n
    println!("You guessed: {}", guess);

    if guess.trim() == number.to_string() {
        println!("You guessed the number!");
    } else {
        println!("Wrong guess. The number was: {}", number);
    }
}
