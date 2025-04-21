const PALABRA: &str = "supercalifragilisticoespialidoso";

fn obtener_palabra() -> Option<String> {
    if !PALABRA.is_empty() {
        Some(PALABRA.to_string())
    } else {
        None
    }
}

fn main() {
    let palabra = obtener_palabra();
    let letras = palabra.map(|p| p.len());

    println!("");
    match letras {
        Some(n) => println!("La palabra tiene {} letras", n),
        None => println!("No hay palabra"),
    }
}
