pub fn _loops() {
    /*
       Se va a ejecutar todo el tiempo sin fin. Es como un while true.
       loop {
           println!("De nuevo!");
       }
    */

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Resultado: {}", result);

    _while_loop();
    _for_loop();
    _fizz_buzz();
}

fn _while_loop() {
    let mut counter = 3;

    while counter != 0 {
        println!("{}!", counter);

        counter -= 1;

        // wait for 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("Despegue!!!!");
}

fn _for_loop() {
    let a = [10, 20, 30, 40, 50];
    let s = "Hola mundo";

    for element in a.iter() {
        println!("El elemento es: {}", element);
    }

    for c in s.chars() {
        println!("El valor es: {}", c);
    }

    for number in 1..4 {
        println!("El valor es: {}", number);
    }
}

fn _fizz_buzz() {
    for number in 1..=100 {
        if number % 3 == 0 && number % 5 == 0 {
            println!("fizzbuzz");
        } else if number % 3 == 0 {
            println!("fizz");
        } else if number % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", number);
        }
    }
}
