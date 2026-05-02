fn action(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Caution"),
        TrafficLight::Green => println!("Go"),
    }
}

fn main() {
    let light = TrafficLight::Yellow;
    action(light);
}
