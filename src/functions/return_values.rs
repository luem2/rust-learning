pub fn _return_values() {
    let x = _returns_three();
    println!("El valor de x es: {}", x);

    let x = _returns_three_explicit();
    println!("El valor de x es: {}", x);

    let y = _sum_two(3, 8);
    println!("El valor de y es: {}", y);

    // Haciendo destructuring
    let (a, b) = _sum_diff(3, 8);
    println!("suma = {}, resta = {}", a, b);

    // Accediendo por indice
    let x = _sum_diff(4, 1);
    println!("suma = {}, resta = {}", x.0, x.1);

    // {:?} se usa para imprimir la tupla
    println!("La suma y la resta es {:?}", x);
}

// Devuelve valor de la funcion, y necesitamos especificar el tipo del dato que queremos devolver.
// No es necesario el "return" (no tiene que tener ; en la ultima linea)
fn _returns_three() -> i32 {
    1 + 2 // return implicito
}

fn _returns_three_explicit() -> i32 {
    let mut num = 1;
    num += 2;

    return num;
}

//
fn _sum_two(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn _sum_diff(n1: i32, n2: i32) -> (i32, i32) {
    (n1 + n2, n1 - n2)
}
