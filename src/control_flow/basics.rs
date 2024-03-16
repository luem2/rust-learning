pub fn main() {
    basic_if();
    other_basic_if();
}

fn basic_if() {
    let condition = true;

    let number = if condition {
        let x = 3;
        x + 5
    } else {
        let x = 2;
        x + 6
    };

    println!("El valor del numero es: {}", number);
}

fn other_basic_if() {
    let num = 15;

    // modulo devuelve el resto de la division
    if num % 2 == 0 {
        println!("{} es par", num);
    } else {
        println!("{} es impar", num);
    }

    if num > 10 {
        println!("{} es mayor que 10", num);
    } else {
        println!("{} es menor que 10", num);
    }
}
