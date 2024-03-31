pub fn _booleans() {
    // Booleans

    let t = true; // declaracion implicita
    let f: bool = false; // declaracion explicita

    println!("verdadero = {}, falso = {}", t, f);

    // if statement
    if t {
        println!("t es verdadero");
    } else {
        println!("t es falso");
    }

    let not_t = !t;

    println!("not_t = {}", not_t);
    /*
        No puedo declarar sin inicializar una variable de tipo boolean.
        Esto me arroja error:

        let b:bool;
    */
}
