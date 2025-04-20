use rand::random;

fn abrir_cofre() -> Result<String, String> {
    let unlock: bool = random();
    if unlock == true {
        Ok(String::from("Tesoro encontrado!"))
    } else {
        Err(String::from("No has podido abrir el cofre"))
    }
}

// fn leer_mensaje(mensaje: String) -> Result<String, String> {
// }
// 
// fn main() {
// }
