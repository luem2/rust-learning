pub fn main() {
    // Tuples
    let tup: (i32, f64, char) = (500, 6.4, 'x');

    // Destructuring
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // Accessing by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let x_char = tup.2;

    println!(
        "five_hundred: {}, six_point_four: {}, x_char: {}",
        five_hundred, six_point_four, x_char
    );
}
