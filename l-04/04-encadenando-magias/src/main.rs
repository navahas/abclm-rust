use rand::random;

fn abrir_cofre() -> Result<String, String> {
    let unlock: bool = random();
    if unlock == true {
        Ok(String::from("Tesoro encontrado!"))
    } else {
        Err(String::from("No has podido abrir el cofre"))
    }
}

fn leer_mensaje(mensaje: String) -> Result<String, String> {
    if mensaje.is_empty() {
        Err("no pudo ser".to_string())
    } else {
        Ok(mensaje)
    }
}

fn main() {
    let result = abrir_cofre().and_then(leer_mensaje);
    println!("@ ----> {:?}", result);
}
