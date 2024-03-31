enum _TrafficLight {
    Red,
    Yellow,
    Green,
}

pub fn _enums() {
    let red_light = _TrafficLight::Red;
    let _yellow_light = _TrafficLight::Yellow;
    let _green_light = _TrafficLight::Green;

    // match es similar a un switch (con sintaxis compacta)
    match red_light {
        _TrafficLight::Red => println!("Pare!"),
        _TrafficLight::Yellow => println!("Baje un cambio!"),
        _TrafficLight::Green => println!("Avance!"),
    }
}
