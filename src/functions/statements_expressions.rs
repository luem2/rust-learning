pub fn _statements_expressions() {
    /* Statements (declaraciones)
        Es una instruccion que realiza una accion pero que no devuelve un valor.
    */

    // Declaracion de variables
    let mut _x: i32 = 42;

    // Asignacion
    _x = 5;

    // Expresiones if else sin asignacion
    if _x != 5 {
        println!("El valor de x es {}", _x);
    }

    // Bucles
    while _x > 0 {
        println!("El valor de x es {}", _x);
        _x -= 1;
    }

    /* Expressions (expresiones):
        Una expresión es una combinación de valores, variables, operadores y funciones que devuelven un valor. Las funciones son consideradas como expresiones, aun aquellas que no devuelvan nada. Es decir, que devuelvan () <- llamada unidad o unit en RUST, que es como el void en otros lenguajes.
    */

    /*
        La x no es un objeto, sino que las llaves son una forma de agrupar un conjunto de instrucciones, es un bloque de codigo usado para:

        - Asignar valores basados en expresiones mas complejas.
        - Delimitar la extension de la vida util de las variables dentro de ese bloque.

        Simplemente evalua la expresion dentro de el y devuelve el resultado de esa expresion.
    */

    let x = {
        let x = 3;
        x + 1
    };

    println!("El valor de x es {}", x);
}
