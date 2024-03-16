pub fn main() {
    // Booleans

    let t = true; // implicit declaration
    let f: bool = false; // explicit declaration

    println!("true = {}, false = {}", t, f);

    // if statement
    if t {
        println!("t is true");
    } else {
        println!("t is false");
    }

    let not_t = !t;

    println!("not_t = {}", not_t);
    /*
        No puedo declarar sin inicializar una variable de tipo boolean.
        Esto me arroja error:

        let b:bool;
    */
}
