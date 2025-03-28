const PI: f64 = std::f64::consts::PI;

fn main() {
    let x = 10;
    println!("\nx: immutable = {}\n", x);

    let mut _y = 11.1;
    _y = 20.0;
    println!("y: After mutation = {}\n", _y);

    let y_shadow = 1.11;
    println!("y_shadow: Before mutation = {}", y_shadow);
    let mut y_shadow = y_shadow as i32;
    println!("y_shadow: After type mutation = {}", y_shadow);
    y_shadow = 20;
    println!("y_shadow: After mutation = {}\n", y_shadow);

    println!("PI: std value = {}", PI);
    println!("PI: Binary = {:b}\n", PI.to_bits());
}
