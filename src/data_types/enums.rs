enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub fn main() {
    let red_light = TrafficLight::Red;
    let _yellow_light = TrafficLight::Yellow;
    let _green_light = TrafficLight::Green;

    // match es similar a un switch (con sintaxis compacta)
    match red_light {
        TrafficLight::Red => println!("Pare!"),
        TrafficLight::Yellow => println!("Baje un cambio!"),
        TrafficLight::Green => println!("Avance!"),
    }
}
