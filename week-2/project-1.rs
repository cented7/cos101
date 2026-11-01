fn main () {
    let loan:f64 = 520000000.00;
    let time:f64 = 5.0;
    let rate:f64 = 10.0;

    let amount:f64 =  loan * (1.0 + (rate/100.0)) * time;

    let ci:f64 = amount - loan;

    println!("The compound interest is {}", ci)
}