fn calcular(a: i32, b: i32, operacion: char) -> Result<i32, String> {
    if b == 0 {
        let error_msg = String::from("No puedes dividir por 0!");
        return Err(error_msg);
    }
    match operacion {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => Ok(a / b),
        _ => Err(String::from("No es una operación válida"))
    }
}

fn main() {
    println!("@ ----> {:?}", calcular(4, 2, '+'));
    println!("@ ----> {:?}", calcular(5, 0, '/'));
}
