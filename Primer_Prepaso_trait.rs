trait Sound {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl Sound for Dog {
    fn make_sound(&self) {
        println!("Guau");
    }
}

impl Sound for Cat {
    fn make_sound(&self) {
        println!("Miau");
    }
}

fn main() {
    let animals: Vec<Box<dyn Sound>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for a in animals {
        a.make_sound();
    }
}
