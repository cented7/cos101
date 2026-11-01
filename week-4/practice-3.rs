use std::io;

fn main () {
    let mut name = String::new();
    let mut age = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("User input failed.");

    println!("Enter your age: ");
    io::stdin().read_line(&mut age).expect("User input failed.");
    let age:i32 = age.trim().parse().expect("Not a valid integer.");

    if age >= 18 {
        println!("Welcome to the party, {}!", name);
    } else {
        println!("You're not of age to enter the party, {}.", name);
    }
}