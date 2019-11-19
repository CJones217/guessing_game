use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let mut secret_number = rand::thread_rng().gen_range(1, 11);
    const MAX_POINTS: u8 = 3;
    let mut correct_times: u8 =0;
    const MAX_ERRORS: u8 = 10;
    let mut wrong_times: u8 =0;

    loop {
        println!("input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {  // defining guess again is an example of shadowing trim removes white space and parse parses string into number (need to say the type of number)
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                wrong_times+=1;
                println!("{} guesses wrong", wrong_times);
            },
            Ordering::Greater => {
                println!("Too big!");
                wrong_times+=1;
                println!("{} guesses wrong", wrong_times);
            },
            Ordering::Equal => {
                correct_times+=1;
                println!("Correct! you correctly guessed {} times",correct_times);
                secret_number = rand::thread_rng().gen_range(1, 11);
            },
        }
        if wrong_times == MAX_ERRORS{
            println!("too many wrong guesses ... you suck!");
            break;
        }
        else if correct_times == MAX_POINTS {
            println!("You guessed correctly {} times! you are cool", correct_times);
            break;
        }
    }
}
