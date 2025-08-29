// sys-math\src\arithmetics.rs

pub fn addition(first: i32, sign: &str, second: i32) -> f64 {
    match sign {
        "+" => (first + second) as f64,
        "-" => (first - second) as f64,
        "/" => first as f64 / second as f64 as f64,
        "*" => (first * second) as f64,
        "%" => (first % second) as f64,
        "**" => first.pow(second as u32) as f64,
        _ => 0.0,
    }
}
