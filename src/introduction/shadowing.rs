pub fn main() {
    let x: i32 = 45;

    {
        let x: i32 = 10;
        println!("El valor de x es: {}", x); // El valor sera 10
    }

    // Podemos declarar nuevamente la misma variable y en el mismo scope con let y no nos arrojara error (Como si pasaria en javascript).
    let x: i32 = x + 20;
    println!("El valor de x es: {}", x); // El valor sera 65

    let x: &str = "Hola mundo!";
    println!("El valor de x es: {}", x);

    // Creamos una variable mutable de tipo i32.
    let mut x: i32 = 10;

    /*
     Esto nos arrojara error, ya que no podemos cambiar el tipo de la variable en la asignacion.
     x = "Hola mundo!"
    */

    x = x + 40;
    println!("El valor de x es: {}", x);
}
