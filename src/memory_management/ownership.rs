pub fn _ownership() {
    let mut name = String::new();
    println!("Ingrese su nombre: ");
    // &mut name es una referencia mutable que apunta a la variable name. Se presta temporalmente el ownership con permiso de escritura.
    std::io::stdin().read_line(&mut name).unwrap();

    // x pasa a ser el dueño del valor que devuelve s.len()
    let x = _size_characters(&name);
    _add_characters(&mut name);

    println!("Tu nombre es: {}, y tiene {} caracteres", name, x);
}

// s pasa a ser el dueño de la variable (temporalmente), recibe la variable con permiso de escritura. La muta y devuelve el ownership. "s" se libera (elimina) de la memoria.
fn _add_characters(s: &mut String) {
    s.push_str("Todo bien?");
}

// s pasa a ser el dueño de la variable (temporalmente), devuelve la longitud de la variable a "x" linea 8 (esta pasa a ser el dueño del retorno de la funcion), luego "s" devuelve el valor a la variable que se la prestó (name).
fn _size_characters(s: &String) -> usize {
    s.len()
}
