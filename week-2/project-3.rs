fn main () {
    let principal:f64 = 210000.00;
    let rate:f64 = 5.00;
    let time = 3;
    
    let amount:f64 = principal * (1.0 - (rate/100.0)).powi(time);

    let depreciation = principal - amount;

    println!("The value after 3 years is {:.2}", depreciation)
}