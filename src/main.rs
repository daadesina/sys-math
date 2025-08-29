// sys-math\src\main.rs

use std::io::{self, Write};
mod algebra;
mod arithmetics;

// The main function
fn main() {
    loop {
        print!("Enter a number between 1 and 3: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Fail to read");

        let option: i32 = input.trim().parse().expect("Option not valid");

        match option {
            1 => arithmetic_main(),
            2 => quadratic_main(),
            3 => factorial_main(),
            _ => panic!("Something Went Wrong!"),
        }
    }
}

//1. Function for Arithmetic
fn arithmetic_main() {
    let mut input = String::new();

    println!("Type Your Arithmetics like 1 + 2: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Fail to read line");

    let mut parts = input.trim().split_whitespace();

    let first: i32 = parts.next().unwrap().parse().expect("Invalid First Number");
    let sign: &str = parts.next().expect("Invalid Sign");
    let second: i32 = parts
        .next()
        .unwrap()
        .parse()
        .expect("Invalid Second Number");

    let result = arithmetics::addition(first, sign, second);
    println!("{} {} {} = {}", first, sign, second, result);
}

// 2. Function for Quadratic Equation
fn quadratic_main() {
    println!("Enter the Quadratic Parameters");
    print!("a = ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid Input");
    let a = input.trim().parse().expect("Invalid");

    print!("b = ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid Input");
    let b = input.trim().parse().expect("Invalid");

    print!("c = ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid Input");
    let c = input.trim().parse().expect("Invalid");

    let result = algebra::quadratic(a, b, c);
    println!("{}x^2 + {}x + {} = {}", a, b, c, result);
}

// 3. Function for Factorial
fn factorial_main() {
    print!("Enter the number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid Input");
    let num = input.trim().parse().expect("Invalid");

    let result = algebra::factorial(num);
    println!("{}! = {} ", num, result);
}
