mod variables;
mod data_types;

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    loop{
        println!();
        println!("{:*>30}","");
        println!("Please select an Option: ");
        println!("{:*>30}","");
        println!();
        println!("{:=>30}","");
        println!("1. Play Guessing Game");
        println!("2. Common Concepts of Programming.");
        println!("Type `exit` to quit");
        println!("{:=>30}","");
        println!("Please Make a Choice: ");

        let input = get_input();

        match check_number(&input) {
            Some(choice) => {
                if choice == 1 {
                    guessing_game()
                } else if choice == 2  {
                    println!();
                    println!("{:->30}","");
                    println!("1. Variables and Mutability.");
                    println!("2. Data Types.");
                    println!("Type `exit` to Return.");
                    println!("{:->30}","");
                    println!();
                    println!("Please Make a Choice: ");

                    let input = get_input();

                    match check_number(&input) {
                        Some(choice) => {
                            if choice == 1 {
                                variables::variables_mutability_1_1();
                            } else if choice == 2 {
                                data_types::data_types_1_2();
                            } else{
                                println!("Invalid choice");
                            }
                        }

                        None => {
                            exit(&input);
                            break;
                        }
                    }

                }else {
                    println!("Invalid choice");
                }
            }

            None => {
                exit(&input);
                break;
            }
        }
    }
}

fn guessing_game() {
    println!();
    println!("Guess the number!");

    let secret_number:u8 = rand::rng().random_range(0..=100);

    //println!("The secret number is: {secret_number}");
    let mut steps: u8 = 0;

    loop {
        println!("Please input your guess: ");

        let guess = get_input();

        let guess: u8 = match check_number(&guess) {
            Some(num) => num as u8,
            None => {
                if guess.trim().eq_ignore_ascii_case("exit") {
                    println!("Goodbye!");
                    break;
                }
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                println!("{:=>30}","{:=>30}");
            },
            Ordering::Greater => {
                println!("Too big!");
                println!("{:=>30}","");
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        steps += 1;
    }
    println!("You guessed in {} guesses!", steps);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn exit(input: &str) {
    let cmd = input.trim();
    if cmd == "exit" {
        println!("Goodbye!");
    }
    println!("You typed: {}", cmd);
}

fn check_number(input:&str) -> Option<i32> {
    if input.trim().eq_ignore_ascii_case("exit") {
        return None;
    }

    match input.trim().parse::<i32>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Please type a number! or exit");
            println!("=========================");
            None
        }
    }
}



