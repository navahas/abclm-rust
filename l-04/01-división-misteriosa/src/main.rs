fn dividir_si_puedes(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        eprintln!("No puedes dividir por 0!");
        return None;
    }

    if a % b == 0 {
        Some(a / b)
    } else {
        None
    }
}

fn main() {
    println!("@ ----> {:?}", dividir_si_puedes(7, 3));
    println!("@ ----> {:?}", dividir_si_puedes(10, 2));
    println!("@ ----> {:?}", dividir_si_puedes(7, 0));
}
