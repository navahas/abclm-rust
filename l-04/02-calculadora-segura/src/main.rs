use std::fmt;

struct Operacion {
    a: i32,
    b: i32,
    operador: char
}

enum Operadores {
    Suma,
    Resta,
    Multiplicacion,
    Division
}

impl Operadores {
    fn simbolo(&self) -> char {
        match self {
            Operadores::Suma => '+',
            Operadores::Resta => '-',
            Operadores::Multiplicacion => '*',
            Operadores::Division => '/',
        }
    }

    fn desde_char(operador: char) -> Option<Operadores> {
        match operador {
            '+' => Some(Operadores::Suma),
            '-' => Some(Operadores::Resta),
            '*' => Some(Operadores::Multiplicacion),
            '/' => Some(Operadores::Division),
            _ => None
        }
    }

    fn aplicar(&self, a: i32, b: i32) -> Result<i32, String> {
        match self {
            Operadores::Suma => Ok(a + b),
            Operadores::Resta => Ok(a - b),
            Operadores::Multiplicacion => Ok(a * b),
            Operadores::Division => {
                if b == 0 {
                    let error_msg = String::from("No puedes dividir por 0!");
                    Err(error_msg)
                } else {
                    Ok(a / b)
                }
            }
        }
    }
}

impl fmt::Display for Operadores {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.simbolo())
    }
}

fn calcular(a: i32, b: i32, operacion: char) -> Result<i32, String> {
    match Operadores::desde_char(operacion) {
        Some(op) => op.aplicar(a, b),
        None => Err(String::from("Operación no permitida"))
    }
}

fn main() {
    let operaciones: Vec<Operacion> = vec![
        Operacion { a: 4, b: 2, operador: '+' },
        Operacion { a: 5, b: 0, operador: '/' },
        Operacion { a: 11, b: 0, operador: '*' },
        Operacion { a: 20, b: 6, operador: '-' },
    ];

    for Operacion { a, b, operador } in operaciones {
        println!("");
        println!("---> calcular({}, {}, '{}')", a, b, operador);
        match calcular(a, b, operador) {
            Ok(res) => println!("{} {} {} = {}", a, operador, b, res),
            Err(err) => eprintln!("Error con '{}':\n{}", operador, err)
        }
    }

}
