use std::io;

fn main () {
   let mut experience = String::new();
   let mut age = String::new();

   println!("Is the user experienced? (yes/no): ");
   io::stdin()
        .read_line(&mut experience)
        .expect("User input failed.");

    println!("Enter your age: ");
    io::stdin()
        .read_line(&mut age)
        .expect("User input failed.");

    let experience = experience.trim().to_lowercase();
    let age:i32 = age.trim().parse().expect("Invalid age.");

    if experience == "yes" && age >= 40 {
        println!("Your annual incentive is NGN 1,560,000.");
    } else if experience == "yes" && age >= 30 && age <= 39 {
        println!("Your annual incentive is NGN 1,480,000.");
    } else if experience == "yes" && age < 28 {
        println!("Your annual incentive is NGN 1,300,000");
    } else if experience == "yes" && age >= 28 && age <= 29 {
        println!("No annual incentive was selected for this age group.");
    } else if experience == "no" {
        println!("Your annual incentive is NGN 100,000");
    } else {
        println!("Not a valid input.");
    }

}