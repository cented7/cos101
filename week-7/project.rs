use std::io;

fn trapezium () {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the height of the trapezium: ");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read user input");

    let input1:f64 = input1.trim().parse().expect("Invalid entry");

    println!("Enter the length of the first base of the trapezium: ");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read user input");
    
    let input2:f64 = input2.trim().parse().expect("Invalid entry");

    println!("Enter the length of the second  base of the trapezium: ");
    io::stdin()
        .read_line(&mut input3)
        .expect("Failed to read user input");
    
    let input3:f64 = input3.trim().parse().expect("Invalid entry");

    let area_trapezium:f64  = (input1 / 2.0) * (input2 + input3);
    
    println!("The area of the trapezium is: {}", area_trapezium);
}

fn rhombus () {
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("Enter the first diagonLal of the rhombus: ");
    io::stdin()
        .read_line(&mut input4)
        .expect("Failed to read user input");

    let input4:f64 = input4.trim().parse().expect("Invalid entry");

    println!("Enter the second diagonal of the rhombus: ");
    io::stdin()
        .read_line(&mut input5)
        .expect("Failed to read user input");
    
    let input5:f64 = input5.trim().parse().expect("Invalid entry");

    let area_rhombus:f64  = 0.5 * input4 * input5;
    
    println!("The area of the rhombus is: {}", area_rhombus);
}

fn parallelogram () {
    let mut input6 = String::new();
    let mut input7 = String::new();

    println!("Enter the length of the base of the parallelogram: ");
    io::stdin()
        .read_line(&mut input6)
        .expect("Failed to read user input");

    let input6:f64 = input6.trim().parse().expect("Invalid entry");

    println!("Enter the height of the parallelogram: ");
    io::stdin()
        .read_line(&mut input7)
        .expect("Failed to read user input");
    
    let input7:f64 = input7.trim().parse().expect("Invalid entry");

    let area_parallelogram:f64  = input6 * input7;
    
    println!("The area of the parallelogram is: {}", area_parallelogram);
}

fn main () {
    println!("The Shape Area Calculator");
    println!("1. Trapezium (T)");
    println!("2. Rhombus (R)");
    println!("3. Parallelogram (P)");

    let mut shape = String::new();
    println!("Select a shape: ");

    io::stdin()
        .read_line(&mut shape)
        .expect("Failed to read user input");
    
    let shape = shape.trim().to_lowercase();

    if shape == "t" {
        trapezium();
    } else if shape == "r" {
        rhombus();
    } else if shape == "p" {
        parallelogram();
    } else {
        println!("Invalid option")
    }
} 