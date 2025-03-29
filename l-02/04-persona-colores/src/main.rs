use std::fmt;

struct Person {
    name: String,
    age: u32,
    fav_color: Color
}

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self {
            Color::Red => "Red",
            Color::Green => "Green",
            Color::Blue => "Blue",
            Color::Yellow => "Yellow"
        };
        write!(f, "{}", color)
    }
}

impl Person {
    fn new(name: &str, age: u32, fav_color: Color) -> Self {
        Self {
            name: name.to_string(),
            age,
            fav_color
        }
    }

    fn show(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Fav Color: {}", self.fav_color);
    }
}

fn main() {
    let users = vec![
        Person::new("Miles Davis", 43, Color::Yellow),
        Person::new("Scott LaFaro", 25, Color::Blue),
        Person::new("Bill Evans", 33, Color::Green),
        Person::new("Paul Motian", 31, Color::Red)
    ];
    
    for user in users {
        println!();
        println!("--- User Details ---");
        user.show();
        println!();
    }
}
