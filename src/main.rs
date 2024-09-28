use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

pub fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
}

pub fn divide(a: i32, b: i32) -> i32 {
    return a / b;
}

pub fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}