use std::io;

fn main () {
    println!("===== RESTAURANT MENU =====");
    println!("P - Poundo Yam / Edinkaiko Soup - ₦3,200");
    println!("F - Fried Rice & Chicken - ₦3,000");
    println!("A - Amala & Ewedu Soup - ₦2,500");
    println!("E - Eba & Egusi Soup - ₦2,000");
    println!("W - White Rice & Stew - ₦2,500");

    println!("\nEnter your food choice: ");

    let mut food = String::new();
    io::stdin()
        .read_line(&mut food)
        .expect("Failed to read user input.");
    let food  = food.trim().to_uppercase();

    println!("Enter quantity: ");

    let mut quantity = String::new();
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read user input.");
    let quantity:i32 = quantity.trim().parse().expect("Not a valid integer.");

    let price = match food.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => 0,
    };

    if price == 0 || quantity <= 0 {
        println!("Invalid price or quantity.")
    } else {
        let total = price * quantity;
        println!("Final total before discount is {}", total);

        if total > 1000 {
            let discount = total * 5/100;
            let final_total = total - discount;

            println!("Discount: NGN {}", discount);
            println!("Final charge: NGN {}", final_total);
        } else {
            println!("No discount applied.");
            println!("Final charge: NGN {}", total);
        }
    } 
}