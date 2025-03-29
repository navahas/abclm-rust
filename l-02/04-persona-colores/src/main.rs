// nombre → tipo string  
// edad → tipo u32
// color favorito → tipo enum.
// rojo, verde, amarillo. además, se deberán añadir datos de prueba y leer la información almacenada en la estructura creada. 

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    fav_color: Colors
}

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Yellow
}

fn main() {
    let user = Person {
        name: String::from("John Doe"),
        age: 33,
        fav_color: Colors::Green
    };

    println!("{:?}", user);
}
