#!/usr/bin/env rust
use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn hello_world() {
    println!("Hello, world!\n");
}

fn get_user_num() -> i64 {
    // Get a number from the user
    print!("Enter a number: ");
    io::stdout().flush().unwrap();  // Send last statement to screen
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    let num: i64 = match trimmed.parse::<i64>() {
        Ok(i) => i,
        Err(..) => { println!("this was not an integer: {}", trimmed); 0 },
    };

    return num;
}

fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess: i64;

    loop {
        println!("The secret number is: {}", secret_number);
        guess = get_user_num();
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            },
        }
    }
}

fn fibonacci() {
    println!("Enter n to get the nth Fibonacci number");
    let n: i64 = get_user_num(); 
    let mut current: i64 = 0;
    let mut next: i64 = 1;
    let mut sum: i64;
    for _i in 1..n+1 {
        sum = next + current;
        current = next;
        next = sum;
    }
    println!("{}th fibonacci number is {}", n, current);    
}

fn main() {
    // Call all functions
    hello_world();
    guessing_game();
    fibonacci();
}
