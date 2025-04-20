#[derive(Debug)]
enum ResultadoDivision {
    Resultado(Option<i32>),
    Error(String)
}

fn dividir_si_puedes(a: i32, b: i32) -> ResultadoDivision {
    if b == 0 {
        let error_msg = String::from("No puedes dividir por 0!");
        return ResultadoDivision::Error(error_msg);
    }

    if a % b == 0 {
        ResultadoDivision::Resultado(Some(a / b))
    } else {
        ResultadoDivision::Resultado(None)
    }
}

impl ResultadoDivision {
    fn print_result(&self, a: i32, b: i32) {
        match self {
            ResultadoDivision::Resultado(Some(res)) => {
                println!("{} / {} = {}", a, b, res);
            },
            ResultadoDivision::Resultado(None) => {
                println!("{} no es divisible exactamente por {}", a, b);
            },
            ResultadoDivision::Error(err) => {
                eprintln!("Error al dividir {} / {}: {}", a, b, err);
            }
        }
    }
}

fn main() {
    let casos: Vec<(i32, i32)> = vec![(10, 2), (7, 3), (11, 0)];
    for (a, b) in casos {
        let resultado = dividir_si_puedes(a, b);
        println!("\n");
        println!("----> {:?}", resultado);
        resultado.print_result(a, b)
    }
}
