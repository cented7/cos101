use std::io;

fn main () {
    let mut lower_bound = String::new();
    let mut upper_bound = String::new();

    println!("Enter the lower bound: ");
    io::stdin().read_line(&mut lower_bound).expect("User input failed!");
    let lower_bound:i32 = lower_bound.trim().parse().expect("Not an integer.");

    println!("Enter the upper bound: ");
    io::stdin().read_line(&mut upper_bound).expect("User input failed!");
    let upper_bound:i32 = upper_bound.trim().parse().expect("Not an integer.");

    for x in lower_bound..upper_bound {
        println!("Count level is {}", x);
    }

}