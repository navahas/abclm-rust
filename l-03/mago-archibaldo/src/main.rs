// HECHIZO CONSTANTE QUE NO CAMBIA
const HECHIZO_DEL_DIA: &str = "Expecto Patronum";

// HECHIZO GLOBAL DISPONIBLE SIEMPRE
static MENSAJE_GLOBAL: &str = "Bienvenido a la biblioteca mágica";

fn main() {
    println!("{}", HECHIZO_DEL_DIA);
    println!("{}", MENSAJE_GLOBAL);

    let libro = String::from("Libro de los Elementos");

    // EL MAGO PRESTA EL LIBRO A DOS APRENDICES Y UNO LO EDITA
    let referencia1 = libro.as_str();
    let referencia2 = libro.as_str();
    let referencia3 = libro.clone();

    println!("{}, {}, {}", referencia1, referencia2, referencia3);

    // RUNA QUE SE COPIA AUTOMÁTICAMENTE
    let runa = 7;
    usar_runa(runa);
    println!("La runa sigue con Archibaldo: {}", runa);

    // EL MAGO ENTREGA EL LIBRO PERO LUEGO QUIERE USARLO
    entregar(&libro);
    println!("El libro aún está con Archibaldo: {}", libro);

    // SE HACE UNA COPIA DEL GRIMORIO ANTES DE ENTREGARLO
    let grimorio = String::from("Grimorio Oscuro");
    entregar(grimorio.as_str());
    println!("Archibaldo aún tiene el grimorio: {}", grimorio);

    // ERROR POR FALTA DE LIFETIME EN LA FUNCIÓN
    let uno = String::from("Fuego");
    let dos = String::from("Fuego y Hielo");
    let resultado = mas_largo(&uno, &dos);
    println!("El libro más largo es: {}", resultado);
}

fn usar_runa(r: i32) {
    println!("La runa usada es: {}", r);
}

fn entregar(libro: &str) {
    println!("Se ha entregado el libro: {}", libro);
}

fn mas_largo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
