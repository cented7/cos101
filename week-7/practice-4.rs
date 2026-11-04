fn main () {
    let numbers:[i32;5] = [3,6,9,12,15];

    for x in numbers {
        println!("{}", x.pow(2));
    }
}