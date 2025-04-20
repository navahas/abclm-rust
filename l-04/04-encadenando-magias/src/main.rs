use rand::random;

fn abrir_cofre() -> Result<String, String> {
    let unlock: bool = random();
}
// 
// fn leer_mensaje(mensaje: String) -> Result<String, String> {
// }

fn main() {
    let x: bool = random();
    println!("@ ----> {}", x);
}
