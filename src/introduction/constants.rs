pub fn main() {
    /*
        Esto si se puede hacer
            let x;
            x = 5;
            println!("x = {}", x);

        Pero no con las constantes (como en javascript)
    */

    // las constantes necesitan un tipo explícito, si no se especifica, arroja un error
    // u32: entero de 32 bits sin signo (es decir, enteros positivos)
    // el guion bajo es a modo de legibilidad, no incluye el valor de la constante
    const MAX_POINTS: u32 = 100_000;

    println!("Max points: {}", MAX_POINTS);
    /*
        No se puede hacer shadowing, esto arroja un error
        const MAX_POINTS: u32 = MAX_POINTS + 10_000;

        Una constante no puede ser mutable
        const mut MAX_POINTS: u32 = 10_000;
    */
}
