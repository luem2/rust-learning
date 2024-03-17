pub fn main() {
    // Characters
    let c = 'z';
    let x = '𝕏'; // El caracter U+1d54f "𝕏" se puede confundir con el caracter ASCII U+0058 "X"
    let heart_eyed_cat = '😻';

    println!("c: {}, x: {}, heart_eyed_cat: {}", c, x, heart_eyed_cat);

    // iterar sobre caracteres en un string
    for char in "Ciao, mondo!, 你好, 👋".chars() {
        println!("{}", char);
    }
}
