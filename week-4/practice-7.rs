use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter your number: ");
    io::stdin().read_line(&mut input).expect("User input failed.");
    let mut num:i32 = input.trim().parse().expect("Not an integer.");

    while num < 10 {
        println!("The count is {}", num);
        num += 1;
    }

    println!("The count is now {}", num);
}