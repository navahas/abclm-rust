struct Libro {
    titulo: &'static str,
    autor: &'static str
}

const LISTA_LIBROS: &[Libro] = &[
    Libro { titulo: "Don Quijote", autor: "Miguel de Cervantes" },
    Libro { titulo: "La sombra del viento", autor: "Carlos Ruiz Zafón" },
    Libro { titulo: "La casa de Bernarda Alba", autor: "Federico García Lorca" },
    Libro { titulo: "Fortunata y Jacinta", autor: "Benito Pérez Galdós" },
    Libro { titulo: "La Regenta", autor: "Leopoldo Alas Clarín" },
    Libro { titulo: "Nada", autor: "Carmen Laforet" },
    Libro { titulo: "Tiempo de silencio", autor: "Luis Martín-Santos" },
    Libro { titulo: "El árbol de la ciencia", autor: "Pío Baroja" },
    Libro { titulo: "Los santos inocentes", autor: "Miguel Delibes" },
    Libro { titulo: "Campos de Castilla", autor: "Antonio Machado" },
];

pub fn buscar_libro(titulo_: &str) -> Option<String> {
    for &Libro { titulo, autor } in LISTA_LIBROS {
        if titulo == titulo_ {
            return Some(String::from(autor))
        }
    }
    None
}
