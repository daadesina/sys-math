// sys-math\src\main.rs

use std::io;
mod arithmetics;

fn main() {
    fn operation() {
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
        println!("{}", result);
    }

    loop {
        operation()
    }
}
