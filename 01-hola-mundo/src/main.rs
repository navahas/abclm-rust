fn main() {
    // Aquí es obligatorio el ; en la macro println! para terminar los statement
    let greet = "Hola, Rust!";
    println!("{}", greet);
        
    // ; no es obligatoria aquí porque la fn main devuele el unit type ()
    // fn main() -> () {} 
    // Similar al void en TypeScript
    println!("Hola de nuevo, Rust!") 
}
