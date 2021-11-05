use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    println!("Please input a number");
    let guess = ask();
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Nailed it"),
    }
}

fn ask() -> u32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse().expect("Please type a number")
}
