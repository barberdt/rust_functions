use std::io;

fn main() {
    // Start fibonacci
    let fib_n = prompt_number_input("Please input the fibonacci index...");
    println!(
        "The fibonacci value for {fib_n} is {}",
        get_fibonacci(fib_n)
    );
    // End fibonacci

    // Start degrees
    let degrees_f = prompt_number_input("Please input the degrees in Fahrenheit...");
    println!(
        "{degrees_f} degrees Fahrenheit is {} degrees Celsius",
        fahrenheit_to_celsius(degrees_f)
    );
    // End degrees

    twelve_days_of_christmas();
}

fn get_fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = b;
        b = a + b;
        a = temp;
    }

    a
}

fn fahrenheit_to_celsius(degrees_f: u32) -> u32 {
    ((degrees_f - 32) * 5) / 9
}

const GIFTS: [&str; 12] = [
    "partidge in a pear tree",
    "turtle doves",
    "french hens",
    "calling birds",
    "golden rings",
    "geese a-laying",
    "swans a-swimming",
    "maids a-milking",
    "ladies dancing",
    "lords a-leaping",
    "pipers piping",
    "drummers drumming",
];

fn twelve_days_of_christmas() {
    for n in 0..12 {
        let gift = GIFTS[n];
    }
}

fn prompt_number_input(prompt_msg: &str) -> u32 {
    let mut input = String::new();
    println!("{}", prompt_msg);
    io::stdin()
        .read_line(&mut input)
        .expect("There was an error reading the line");

    let input = input
        .trim()
        .parse()
        .expect("Your input was not a valid number");

    input
}
