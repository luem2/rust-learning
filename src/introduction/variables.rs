pub fn main() {
    let x: i32 = 5;
    println!("El valor de x es: {}", x);

    /*
    Cuando declaramos una variable con let, el valor es inmutable
    esto arrojara un error:
    x = 6;
    println!("El valor de x es: {}", x);
    */

    // Para declarar una variable mutable, usamos la keyword "mut"
    let mut x: i32 = 15;
    println!("El valor de x es: {}", x);

    x = 30;
    println!("El valor de x es: {}", x);
}
