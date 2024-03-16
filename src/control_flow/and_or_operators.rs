pub fn main() {
    let a = 10;
    let b = 5;
    let c = 20;

    // Usando && (and) para comparar si 'a' es mayor que 'b' y 'a' es mayor que 'c'
    if a > b && a > c {
        println!("{} es el mayor de los tres valores", a);
    } else {
        println!("Condicion && no se cumple");
    }

    // Usando || (or) para comparar si 'a' es mayor que 'b' o 'a' es mayor que 'c'
    if a > b || a > c {
        println!("Al menos una de las dos condiciones se cumple");
    } else {
        println!("Condicion || no se cumple");
    }
}
