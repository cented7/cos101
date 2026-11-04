use std::io;

fn checker () {
    let mut input = String::new();
    println!("Enter a character: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input.");
    let ch:char = input.trim().parse().expect("Invalid character.");

    if ch >= '0' && ch <= '9' {
        println!("Character {} is a digit.", ch);
    } else {
        println!("Character {} is not a digit.", ch);
    }
}

fn main () {
    println!("This program checks if your input is a digit or not.");
    checker();
}