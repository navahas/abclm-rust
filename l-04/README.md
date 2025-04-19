## Lección 4 (Final)

### Ejercicio 1: La división misteriosa
Crea una función
```rust
dividir_si_puedes(a: i32, b: i32) -> Option<i32>```
Debe devolver `Some(resultado)` si la división es exacta, y `None` si no lo es o si b == 0.
Ejemplo:
```rust
  dividir_si_puedes(10, 2)  Some(5)
  dividir_si_puedes(7, 3)  None
```
- Bonus: imprime un mensaje divertido si no se puede dividir.

---
### Ejercicio 2: Calculadora segura
Crea una función
```rust
calcular(a: i32, b: i32, operacion: char) -> Result<i32, String>.
```
Debe soportar: '+', '-', '*', '/'.
Si se intenta dividir por 0, debe devolver un error con un mensaje personalizado.
Ejemplo:
```rust
  calcular(4, 2, '+')  Ok(6)
  calcular(5, 0, '/')  Err("No se puede dividir entre cero!")
```

--- 
### Ejercicio 3: Biblioteca secreta
Crea un archivo llamado `biblioteca.rs` con una función pública.
```rust
buscar_libro(titulo: &str) -> Option<String>.
```
Si el libro está en una lista interna, devuelve su autor. Si no está, devuelve `None`.
- Bonus: Haz que la lista de libros est en una constante privada dentro del módulo.

---
### Ejercicio 4: Encadenando magias
Imagina que tienes las siguientes funciones.
```rust
abrir_cofre() -> Result<String, String>
leer_mensaje(mensaje: String) -> Result<String, String>
```
Utiliza `.and_then()` para obtener el mensaje final solo si todo va bien.
Ejemplo:
```rust
abrir_cofre() -> Ok("Tesoro encontrado!")
leer_mensaje("Tesoro encontrado!") -> Ok("El mensaje es: Sigue aprendiendo Rust")
```

---
### Ejercicio 5: El adivinador de palabras
Crea una función que devuelva una palabra opcional `(Option<String>)`, y luego usa `.map()` para:
- Contar las letras si existe
- Imprimir el número de letras o un mensaje si no hay palabra
Ejemplo:
```rust
let palabra = obtener_palabra();
let letras = palabra.map(|p| p.len())
```

