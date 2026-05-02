use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let n = 5; // cantidad de autos

    // El puente es un recurso exclusivo
    let puente = Arc::new(Mutex::new(()));

    for i in 0..n {
        let p = puente.clone();

        // dirección del auto: 0 = norte, 1 = sur
        let direccion = if i % 2 == 0 { "Norte" } else { "Sur" };

        thread::spawn(move || {
            println!("Auto {i} quiere cruzar desde {direccion}");

            // tomar el puente
            let _lock = p.lock().unwrap();

            println!("Auto {i} está cruzando el puente desde {direccion}");
            thread::sleep(Duration::from_millis(500));

            println!("Auto {i} terminó de cruzar");
        });
    }

    // evitar que main termine
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
