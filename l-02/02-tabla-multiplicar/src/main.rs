fn main() {
    let first_arg = std::env::args().nth(1).expect("provide any number");
    let number: usize = first_arg.parse().unwrap();
    let mut count = 1;
    println!("- Multiplication Table");
    println!("-------------------------");
    while count < 11 {
        println!("-- {} x {} = {}", count, number, number * count);
        count += 1;
    }
}
