mod biblioteca;
use biblioteca::buscar_libro;

fn main() {
    let titulos = [
        "El árbol de la ciencia",
        "Don Quijote",
        "El señor de los anillos",
        "La sombra del viento",
        "La casa de Bernarda Alba",
        "La historia interminable"
    ];

    for titulo in titulos {
        match buscar_libro(titulo) {
            Some(autor) => println!("---> Disponible:\n- Titulo: {}\n- Author: {}\n", titulo, autor),
            None => println!("---> No Disponible:\n- Titulo: {}\n", titulo)
        }
    }
}
