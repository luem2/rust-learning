struct Person {
    name: String,
    age: u8,
}

pub fn main() {
    // Structs are used to group related data together (use in the top-level file)

    let person = Person {
        name: String::from("John"),
        age: 30,
    };

    println!("{} is {} years old", person.name, person.age);
}
