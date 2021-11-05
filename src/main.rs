use rand::Rng;
use std::cmp::Ordering;
use std::io;

enum GameErr {
    InputErr(String),
}
type InputResult = Result<u32, GameErr>;

fn main() {
    println!("Guess!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input a number");
        let guess: u32 = match ask() {
            Ok(x) => x,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Nailed it");
                break;
            }
        }
    }

    println!("Exiting now");
}

fn ask() -> InputResult {
    let mut guess = String::new();
    let _ = match io::stdin().read_line(&mut guess) {
        Ok(_) => (),
        Err(e) => return Err(GameErr::InputErr(e.to_string())),
    };
    match guess.trim().parse() {
        Ok(x) => Ok(x),
        Err(e) => Err(GameErr::InputErr(e.to_string())),
    }
}
