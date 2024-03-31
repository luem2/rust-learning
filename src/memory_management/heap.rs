pub fn _heap() {
    /*
    Se almacenan datos dinamicos, no se conoce el tamaño y la duración es más larga.
    Los datos en el heap se asignan dinámicamente y se accede a través de punteros.

    En Rust, los datos en el heap se gestionan utilizando tipos de datos como Box<T>, Vec<T>, String, etc.

    La asignación y liberación de memoria en el heap son explícitas en Rust. El programador debe llamar a funciones especiales como Box::new(), Vec::new(), String::new() para asignar memoria en el heap y utilizar la palabra clave drop o dejar que los valores se desechen automáticamente cuando salgan del ámbito para liberar la memoria.
    */

    let _s = String::from("Holaaa"); // creamos un string inmutable, por ende se almacena en el stack..
    let mut _s = String::from("Adios"); // creamos un string mutable, por ende se almacena en el heap.
}
