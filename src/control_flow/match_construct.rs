// con enum, podemos definir un tipo de dato con un conjunto de valores fijos.
enum _Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn _value_in_cents(coin: _Coin) -> u8 {
    match coin {
        _Coin::Penny => 1,
        _Coin::Nickel => 5,
        _Coin::Dime => 10,
        _Coin::Quarter => 25,
    }
}

pub fn _match_constructor() {
    // definimos las monedas
    let (penny, nickel, dime, quarter) = (_Coin::Penny, _Coin::Nickel, _Coin::Dime, _Coin::Quarter);

    // imprimimos el valor de las monedas
    println!("El valor de penny es: {}", _value_in_cents(penny));
    println!("El valor de nickel es: {}", _value_in_cents(nickel));
    println!("El valor de dime es: {}", _value_in_cents(dime));
    println!("El valor de quarter es: {}", _value_in_cents(quarter));
}
