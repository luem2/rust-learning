struct Person {
    name: String,
    age: u8,
}

pub fn main() {
    // Las estructuras se utilizan para agrupar datos relacionados (se pone al principio del archivo)

    let person = Person {
        name: String::from("John"),
        age: 30,
    };

    println!("{} is {} years old", person.name, person.age);
}
