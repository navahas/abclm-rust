use std::env;
use std::process;

fn get_input_args() -> Option<String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        None
    } else {
        Some(args.join(" "))
    }
}

fn main() {
    match get_input_args() {
        Some(input) => {
            println!("{}", &input.to_uppercase());
        },
        None => {
            eprintln!("Error: No argument provided!");
            process::exit(1)
        }
    }
}
