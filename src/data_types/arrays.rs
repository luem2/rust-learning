pub fn main() {
    // Arrays: coleccion de datos del MISMO TIPO con el TAMAÑO FIJO

    let arr = [1, 2, 3, 4, 5]; // array de 5 elementos

    let first = arr[0];
    let second = arr[1];

    println!("first: {}, second: {}", first, second);

    for element in arr.iter() {
        println!("{}", element);
    }
}
