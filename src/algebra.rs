pub fn quadratic(a: f64, b: f64, c: f64) -> String {
    let first_result = (-b + (b.powi(2) - (4.0 * a * c)).sqrt()) / (2.0 * a);
    let second_result = (-b - (b.powi(2) - (4.0 * a * c)).sqrt()) / (2.0 * a);

    format!("{} or {}", first_result, second_result)
}

pub fn factorial(a: i32) -> i32 {
    if a < 0 {
        panic!("Invalid Number")
    } else if a == 0 || a == 1 {
        1
    } else {
        a * factorial(a - 1)
    }
}
