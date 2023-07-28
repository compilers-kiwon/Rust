#[derive(Clone)]
struct Person {
    name: String,
    age : i64,
}

impl Person {
    fn new(name: &str, age: i64) -> Self {
        Person {name: name.to_string(), age,}
    }
}

fn main() {
    let alex = Person::new("Alex", 18);
    let mut betty = alex.clone();
    betty.name = "Betty".to_string();
    println!("{}, {}", alex.name, alex.age);
    println!("{}, {}", betty.name, betty.age);
}