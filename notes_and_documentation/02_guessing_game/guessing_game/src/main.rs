// I personally do not like using "use" statements at the beginning of files
// if I can
// use std::io; // bringing the io crate in from STL for rust
use rand::Rng; // this must be in scope for us to use rand methods
use std::io::Write; // need for flushing

fn main() {
    println!("Guess the number!");

    let difficulty = select_difficulty(
        "Select difficulty:\n1) 1-100\n2) 1-1000\n3) 1-10000\nChoice: ",
        "Difficulty");
    if difficulty < 1 || difficulty > 3 {
        println!("Incorrect value for difficulty!");
        return;
    }

    let tries = select_difficulty(
        "Select Number of tries:\n1) 25 tries\n2) 10 tries\n3) 5 tries\n4) 1 tries\nChoice: ",
        "Tries");
    if tries < 1 || tries > 4 {
        println!("Incorrect value for # of tries!");
        return;
    }

    let mut current_tries = 0u8;
    let tries: u8 = decode_tries(&tries);
    // use a random number in current thread of OS
    let top: u32 = decode_difficulty(&difficulty);
    let secret_number: u32 = rand::thread_rng().gen_range(1..=top);
    loop {
        if current_tries >= tries {
            println!("You lost...");
            break;
        }

        let guessing_game_result: i8 = guessing_game(&secret_number);
        match guessing_game_result {
            -1 => println!("You guessed too low."),
            0 => {
                println!("You WON!");
                break;
            }
            1 => {println!("You guessed too high.");}
            _ => {
                println!("Invalid value!");
                return;
            }
        }

        current_tries += 1;
    }
}

fn select_difficulty(message: &str, error_message: &str) -> u32 {
    let mut difficulty = String::new();

    print!("{}", message);

    // need to flush to print on same line as input?
    let _flush = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line!");

    // need to use &() for the string below since expect() wants a string reference!
    // Need to use to_string() for appending strings together
    let difficulty: u32 = difficulty.trim().parse().expect(&(error_message.to_string() + " must be a number!"));
    return difficulty;
}

fn decode_difficulty(difficulty: &u32) -> u32 {
    match difficulty {
        1 => return 100,
        2 => return 1000,
        3 => return 10000,
        _ => return 10,
    }
}

fn decode_tries(tries: &u32) -> u8 {
    match tries {
        1 => return 25,
        2 => return 10,
        3 => return 5,
        4 => return 1,
        _ => return 99,
    }
}

fn guessing_game(secret_number: &u32) -> i8 {
    println!("Input your guess:");

    // variables are immutable by default - meaning that once we give the
    // variable a value, the value will not change
    let mut guess = String::new();

    // need the mut keyword here since we are "mutating" a variable
    // & means that we are passing in a reference - just like C
    /* 
        references are also immutable by default - but read_line wants to
        change the variable, so we need to specify mutability
    */
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    // We can use this format to put variables only in print macros?
    println!("You guessed: {guess}");

    // Shadowing is acceptable in Rust?
    let guess: u32 = guess.trim().parse().expect("Guess MUST be a number!");

    if guess < *secret_number {
        return -1;
    } else if guess == *secret_number {
        return 0;
    } else {
        return 1;
    }
}

fn _example_functions(guess: &u32) {
    // need to use trim() to cut off newline
    // need to dereference to use value - even for immutables!
    let guess_num: u32 = *guess;
    _normal_ref_example(&guess_num, "");

    let mut change_guess: u32 = guess_num;
    _mutable_ref_example(&mut change_guess);
    _normal_ref_example(&change_guess, " (after multiplying)");
}

// the &i32 means that this function is expecting an immutable reference
fn _normal_ref_example(test: &u32, message: &str) {
    println!("Here is your test value{message}: {test}");
}

fn _mutable_ref_example(mut_test: &mut u32) {
    // need to dereference like in C to change this variable
    *mut_test *= 2;
}
