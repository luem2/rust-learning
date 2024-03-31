pub fn _move_clone() {
    let s = String::from("Holaaa");
    let s1 = s;

    // Esto arrojara error, ya que s ya no existe, porque se le movio el ownership a s1
    // println!("{}", s);

    println!("{}", s1);

    let _s2 = s1.clone();
    // no hay problemas, ya que copiamos el valor de s1 y no se le movio el ownership
    println!("{}", s1);
}
