fn main() {
    let day_number = std::env::args().nth(1).expect("You must provide a number");
    let day_week = match day_number.parse::<u8>().unwrap() {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Not a valid day number"
    };
    println!("{}", day_week)
}
