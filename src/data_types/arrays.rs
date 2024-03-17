pub fn main() {
    // Arrays: coleccion de datos del MISMO TIPO con el TAMAÑO FIJO

    let arr = [1, 2, 3, 4, 5]; // arreglo de 5 elementos

    let first = arr[0];
    let second = arr[1];

    println!("primero: {}, segundo: {}", first, second);

    for element in arr.iter() {
        println!("{}", element);
    }
}
