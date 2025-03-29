fn odd_or_even(number: usize) -> bool {
    let result: bool = if number % 2 == 0 {
        true
    } else {
        false
    };
    return result
}

fn main() {
    let args = std::env::args().nth(1).expect("provide a number");
    let number: usize = args.parse().unwrap();
    let msg = if odd_or_even(number) {"par"} else {"impar"};
    println!("{}", msg)
}
