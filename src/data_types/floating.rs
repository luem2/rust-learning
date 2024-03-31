pub fn _floating() {
    // Floating

    let x = 2.0; // f64 default (pero si la uso en una operacion con otro flotante de f32, este es inferido a f32)
    let y: f32 = 3.0; // f32

    let z = 5.0; // f64 default
                 //
    println!("{} {} {}", x, y, z);

    // Numeric Operations
    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let quotient = x / y;

    println!(
        "sum:{} difference:{} product:{} quotient:{}",
        sum, difference, product, quotient
    );
}
