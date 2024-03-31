pub fn _stack() {
    /*
    Solo se almacenan datos, con tamaño conocido y de corta duración. La forma en la que se almacenan con el metodo (LIFO) Last in First Out.

    En Rust, las variables primitivas como enteros, booleanos, punteros y referencias a datos se almacenan en el stack.

    La asignación y liberación de memoria en el stack son automáticas y están gestionadas por el compilador de Rust, lo que significa que no es necesario que el programador se preocupe por liberar la memoria manualmente.
    */

    // Variable hola aqui no existe
    let hola: &str = "hola"; // se crea hola y al ser inmutable, va a stack.
    println!("{}", hola); // se usa hola

    /* variable hola deja de estar en uso, se elimina de memoria */
}
