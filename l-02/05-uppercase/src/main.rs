use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1..].join(" ").to_uppercase());
}
