#[derive(Debug)]
struct ResultadoDivision {
    valor: Option<i32>,
    mensaje: Option<String>,
    error: Result<bool, bool>
}

fn dividir_si_puedes(a: i32, b: i32) -> ResultadoDivision {
    if b == 0 {
        let error_msg = String::from("No puedes dividir por 0!");
        return ResultadoDivision {
            valor: None,
            mensaje: Some(error_msg),
            error: Err(true)
        }
    }

    if a % b == 0 {
        let res = Some(a / b);
        ResultadoDivision {
            valor: res,
            mensaje: None,
            error: Ok(true)
        }
    } else {
        let error_msg = format!("{} no es divisible sin resto por {}", a, b);
        ResultadoDivision {
            valor: None,
            mensaje: Some(error_msg),
            error: Err(true)
        }
    }
}

fn main() {
    let num_tup: Vec<(i32, i32)> = vec![(10, 2), (7, 3), (11, 0)];
    for (a, b) in num_tup {
        let result = dividir_si_puedes(a, b);
        println!("{:?} ----> {:?}, {:?}", result.error, result.valor, result.mensaje)
    }
}
