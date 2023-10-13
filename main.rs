// question 1
fn main() {
    println!("Hello, world!");
}

//question 2
fn main(){
    println!("Hassan Munir");
    println!("cs201181");
}

//question 3
use std::io;

fn main() {
    println!("Temperature Converter");

    println!("Enter the temperature value:");
    let mut input_value = String::new();
    io::stdin().read_line(&mut input_value).expect("Failed to read line");
    let input_value: f64 = input_value.trim().parse().expect("Invalid input. Please enter a number.");

    println!("Enter the scale (C or F):");
    let mut scale = String::new();
    io::stdin().read_line(&mut scale).expect("Failed to read line");
    let scale = scale.trim().to_lowercase();

    match scale.as_str() {
        "c" => {
            let converted_temp = (input_value * 9.0 / 5.0) + 32.0;
            println!("Converted temperature: {}°F", converted_temp);
        }
        "f" => {
            let converted_temp = (input_value - 32.0) * 5.0 / 9.0;
            println!("Converted temperature: {}°C", converted_temp);
        }
        _ => println!("Invalid scale. Please enter 'C' or 'F'."),
    }
}

//question 4
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let number = 5; // You can change this number to calculate the factorial of a different value
    let result = factorial(number);
    println!("Factorial of {} is: {}", number, result);
}

//question 5
use std::io;

fn main() {
    println!("Enter an integer:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Invalid input. Please enter an integer.");

    if number % 2 == 0 {
        println!("The number {} is even.", number);
    } else {
        println!("The number {} is odd.", number);
    }
}
//question 6
use std::io;

fn main() {
    println!("Enter a positive integer N:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u64 = input.trim().parse().expect("Invalid input. Please enter a positive integer.");

    let sum = calculate_sum(n);
    println!("The sum of all integers from 1 to {} is: {}", n, sum);
}

fn calculate_sum(n: u64) -> u64 {
    (1..=n).sum()
}
//question7
fn main() {
    for num in 1..=100 {
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", num);
        }
    }
}
//question 8
fn is_palindrome(input: &str) -> bool {
    let reversed_string: String = input.chars().rev().collect();
    input == reversed_string
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    if is_palindrome(input) {
        println!("The string is a palindrome.");
    } else {
        println!("The string is not a palindrome.");
    }
}
// question 9
extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Welcome to the Guess the Number Game!");
    println!("I have selected a random number between 1 and 100.");

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too low! Try again."),
            std::cmp::Ordering::Greater => println!("Too high! Try again."),
            std::cmp::Ordering::Equal => {
                println!("Congratulations! You guessed the correct number: {}", secret_number);
                break;
            }
        }
    }
}
// question 10
use std::io;

fn main() {
    println!("Welcome to the Basic Calculator!");

    println!("Enter the first number:");
    let num1: f64 = get_user_input();

    println!("Enter the second number:");
    let num2: f64 = get_user_input();

    println!("Enter the operation (+, -, *, /):");
    let operation: char = get_operation();

    let result = match operation {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Error: Division by zero is not allowed.");
                return;
            }
        }
        _ => {
            println!("Invalid operation entered.");
            return;
        }
    };

    println!("Result: {}", result);
}

fn get_user_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid input. Please enter a number.")
}

fn get_operation() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().chars().next().expect("Invalid input. Please enter an operation.")
}
//question 11
fn find_largest_number(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        None
    } else {
        Some(*numbers.iter().max().unwrap())
    }
}

fn main() {
    let numbers = [10, 5, 8, 22, 14, 17];
    
    match find_largest_number(&numbers) {
        Some(largest) => println!("The largest number is: {}", largest),
        None => println!("The array is empty."),
    }
}
//question 12
fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    let reversed_string = reverse_string(input);
    println!("Reversed string: {}", reversed_string);
}
//question 13
fn generate_fibonacci_sequence(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        sequence.push(a);
        let temp = a;
        a = b;
        b += temp;
    }

    sequence
}

fn main() {
    println!("Enter the number of terms for Fibonacci sequence:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Invalid input. Please enter a number.");

    let fibonacci_sequence = generate_fibonacci_sequence(n);
    println!("Fibonacci sequence up to {} terms: {:?}", n, fibonacci_sequence);
}
// question 14
fn primes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for num in 2..=(limit as f64).sqrt() as usize {
        if is_prime[num] {
            for multiple in (num * num..=limit).step_by(num) {
                is_prime[multiple] = false;
            }
        }
    }

    let primes: Vec<usize> = is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i) } else { None })
        .collect();

    primes
}

fn main() {
    let n = 500;
    let primes = primes(n);

    println!("Prime numbers up to {}: {:?}", n, primes);
}
//question 15
fn modify_string(mut input_string: String) -> String {
    input_string.push_str(", modified");
    input_string
}

fn main() {
    let original_string = String::from("Hello, ownership");
    println!("Original String: {}", original_string);

    let modified_string = modify_string(original_string);
    println!("Modified String: {}", modified_string);

}
//question 16
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    let string1 = "Rust is powerful";
    let string2 = "Borrow checker is strict";

    let longest_string = longest(string1, string2);
    println!("The longest string is: {}", longest_string);
}
//question 17