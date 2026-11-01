fn main () {
    let toshiba:f64 = 450000.00;
    let mac:f64 = 1500000.00;
    let hp:f64 = 750000.00;
    let dell:f64 = 2850000.00;
    let acer:f64 = 250000.00;

    let sum:f64 = toshiba + mac + hp + dell + acer;

    let avg:f64 = sum/5.0;

    println!("The sum is {}", sum);
    println!("The average is {}", avg);
}