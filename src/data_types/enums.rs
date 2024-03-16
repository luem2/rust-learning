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
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Slow down!"),
        TrafficLight::Green => println!("Go!"),
    }
}
