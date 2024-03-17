pub fn main() {
    println!("Hola mundo!");
    /*
        Podemos utilizar la funcion declarada abajo (no importa el orden)
        Esto funciona porque, el compilador de rust, analiza todas las funciones antes de compilar el codigo principal, permitiendo asi, encontrar la definicion de la misma.
    */

    print_number(42);
    many_parameters(42, 'a');
}

fn print_number(num: i32) {
    println!("El número es: {}", num);
}

fn many_parameters(num: i32, cr: char) {
    println!("El número es: {} y el caracter es: {}", num, cr);
}

/* Parametros y argumentos
    - Parámetro:
    Un parámetro es un nombre que se utiliza en la definición de una función para referirse a un valor que se espera que se pase a la función cuando se la llame.
    Los parámetros se definen en la declaración de la función entre paréntesis y consisten en un nombre de parámetro seguido opcionalmente por su tipo.
    Los parámetros actúan como variables locales dentro del cuerpo de la función y representan los valores que se pasan a la función cuando se la llama.
    Por ejemplo, en la función sumar(a: i32, b: i32), a y b son parámetros que representan los valores enteros que se pasan a la función para ser sumados.

    - Argumento:
    Un argumento es el valor real que se pasa a una función cuando se la llama.
    Los argumentos se proporcionan entre paréntesis al llamar a una función y pueden ser valores literales, variables u expresiones que producen un valor compatible con el tipo esperado por los parámetros de la función.
    Cuando se llama a una función, los argumentos se corresponden con los parámetros según su posición en la lista de parámetros de la función.
    Por ejemplo, en la llamada a la función sumar(5, 3), 5 y 3 son argumentos que se pasan a la función para sumarse.
*/
