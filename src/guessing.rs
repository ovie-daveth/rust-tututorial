use std::io;
use rand::Rng;

pub fn guessing() {
    println!("Guess the number!");

    let mut attempt = 5;
    let secret_number = rand::thread_rng().gen_range(1..101); // Correct range for random number

    println!("Please input your guess");

    loop {
        let mut guess = String::new();

        println!("Enter your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess to an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Check if the guess is correct
        if guess == secret_number {
            println!("You won! Yayy");
            break;
        } else {
            attempt -= 1;
            if attempt > 0 {
                println!("Incorrect guess. You have {} attempts remaining.", attempt);
            } else {
                println!("No attempts left. The secret number was {}. Better luck next time!", secret_number);
                break;
            }
        }
    }
}
