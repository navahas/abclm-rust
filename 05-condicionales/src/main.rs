use std::io;

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn sub(x: f64, y: f64) -> f64 {
    x - y
}

fn mul(x: f64, y: f64) -> f64 {
    x * y
}

fn div(x: f64, y: f64) -> Option<f64> {
    if y != 0.0 {
        Some(x / y)
    } else {
        None
    }
}

fn verify_operator(x: f64, operator: &str, y: f64) -> Option<f64> {
    match operator {
        "+" => Some(add(x, y)),
        "-" => Some(sub(x, y)),
        "*" => Some(mul(x, y)),
        "/" => div(x, y),
        _ => None,
    }
}

fn main() {
    println!("Welcome to the Calculator!\n- Enter an expression with the following valid operators: +, -, *, /");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() > 3 {
        println!("Invalid input! Use format: number operator number (e.g., 5 + 3)");
        return;
    }


    let x = parts[0].parse::<f64>();
    let y = parts[2].parse::<f64>();
    let operator = parts[1];

    match (x, y) {
        (Ok(x), Ok(y)) => {
            match verify_operator(x, operator, y){
                Some(result) => println!("Result: {}", result),
                None => println!("Error! Invalid operator or div by 0")
            }
        }
        _ => println!("Error! Invalid number input")
    }
}
