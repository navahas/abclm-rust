use rand::random;
use std::{thread, time};

struct Cofre {
    unlock: bool,
    mensaje: String
}

impl Cofre {
    fn nuevo() -> Self {
        Cofre {
            unlock: random(),
            mensaje: "Sigue aprendiendo Rust".to_string()
        }
    }

    fn abrir_cofre(&self) -> Result<String, String> {
        if self.unlock == true {
            Ok(String::from("Tesoro encontrado!"))
        } else {
            Err(String::from("No has podido abrir el cofre"))
        }
    }

    fn leer_mensaje(&self) -> Result<String, String> {
        let msg = format!("El mensaje es: {}", self.mensaje);
        match self.unlock {
            true => Ok(msg),
            false => Err("El cofre esta vacío!".to_string())
        }
    }

}

fn main() {
    let mut attempts: u8 = 1;
    let mut resultado: Result<String, String>;

    loop {
        let cofre = Cofre::nuevo();
        resultado = cofre
            .abrir_cofre()
            .and_then(|_| cofre.leer_mensaje());

        println!("");
        println!("{:?}", resultado);
        if resultado.is_ok() || attempts >= 3 {
            break;
        }

        println!("Intentando abrir el cofre de nuevo..... {}/3 intentos", attempts);
        attempts += 1;

        thread::sleep(time::Duration::from_secs(2))
    }

}
