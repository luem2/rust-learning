// con enum, podemos definir un tipo de dato con un conjunto de valores fijos.
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn main() {
    // definimos las monedas
    let (penny, nickel, dime, quarter) = (Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter);

    // imprimimos el valor de las monedas
    println!("El valor de penny es: {}", value_in_cents(penny));
    println!("El valor de nickel es: {}", value_in_cents(nickel));
    println!("El valor de dime es: {}", value_in_cents(dime));
    println!("El valor de quarter es: {}", value_in_cents(quarter));
}
