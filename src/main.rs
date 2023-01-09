use std::{io, str::FromStr};

fn main() {
    temperature_coverter_program();

    println!("{}", "-".repeat(50));

    fibonacci_program();

    println!("{}", "-".repeat(50));

    twelve_days_of_christmas();
}

fn temperature_coverter_program() {
    println!("Type temperature in Fahrenheit:");
    let temp_f: f64 = parse_from_stdin("Temperature must be a decimal number, please type again:");
    let temp_c = farenheit_to_celsius(temp_f);
    println!("{temp_f}F = {temp_c}C");
}

fn farenheit_to_celsius(temp_f: f64) -> f64 {
    ((temp_f - 32.0) * 5.0) / 9.0
}

fn fibonacci_program() {
    println!("Type fibonacci index:");
    let nth: u64 = parse_from_stdin("Index must be a natural number, please type again:");
    let fib = naive_fibonacci(nth);
    println!("fibonacci({nth}) = {fib}");
}

fn naive_fibonacci(index: u64) -> u64 {
    if index <= 1 {
        return 1;
    }
    naive_fibonacci(index - 1) + naive_fibonacci(index - 2)
}

fn parse_from_stdin<T: FromStr>(parse_error_msg: &str) -> T {
    // keep looping and asking for input until it parses to T sucessfully
    loop {
        let mut from_stdin = String::new();

        // if it fails to read from stdin, there's not much we can do: panic
        io::stdin()
            .read_line(&mut from_stdin)
            .expect("Failed to read line");

        match from_stdin.trim().parse::<T>() {
            Ok(val) => {
                // parse succeeded: break with parsed value
                break val;
            }
            Err(_) => {
                // parse failed: ask for input and loop
                println!("{}", parse_error_msg);
                continue;
            }
        }
    }
}

const CHRISTMAS_PRESENTS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle-doves",
    "three French hens",
    "four calling birds",
    "five golden rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

fn format_day(day: usize) -> String {
    match day {
        1 => String::from("1st"),
        2 => String::from("2nd"),
        3 => String::from("3rd"),
        val => format!("{val}th"),
    }
}

fn print_lyrics_for_day(day: usize) {
    let mut index = day;
    while index > 0 {
        index -= 1;
        if day > 1 && index == 0 {
            println!("and {}", CHRISTMAS_PRESENTS[index]);
        } else {
            println!("{}", CHRISTMAS_PRESENTS[index]);
        }
    }
}

fn twelve_days_of_christmas() {
    for day in 1usize..=12usize {
        println!("on the {} day of christmas", format_day(day));
        println!("my true love sent to me");
        print_lyrics_for_day(day);
        print!("\n");
    }
}
