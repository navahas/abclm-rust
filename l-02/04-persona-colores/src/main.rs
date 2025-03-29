use std::fmt;

struct Person {
    name: String,
    age: u32,
    fav_color: Color
}

enum Color {
    Red,
    Green,
    Yellow
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self {
            Color::Red => "Red",
            Color::Green => "Green",
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
    let user = Person::new("John Doe", 33, Color::Yellow);
    user.show();
}
