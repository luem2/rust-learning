pub fn main() {
    // Characters

    let c = 'z';
    let x = '𝕏'; // The character U+1d54f "𝕏" could be confused with the ASCII character U+0058 "
    let heart_eyed_cat = '😻';

    println!("c: {}, x: {}, heart_eyed_cat: {}", c, x, heart_eyed_cat);

    // iterate over characters in a string

    for char in "Ciao, mondo!, 你好, 👋".chars() {
        println!("{}", char);
    }
}
