use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    // Monitor = (Mutex<Estado>, Condvar)
    let monitor = Arc::new((
        Mutex::new(Estado { buffer: Vec::new(), capacidad: 5 }),
        Condvar::new(),
    ));

    // PRODUCTOR
    {
        let m = monitor.clone();
        thread::spawn(move || {
            let mut item = 0;
            loop {
                let (lock, cond) = &*m;
                let mut estado = lock.lock().unwrap();

                // Esperar si el buffer está lleno
                while estado.buffer.len() == estado.capacidad {
                    estado = cond.wait(estado).unwrap();
                }

                // Producir
                estado.buffer.push(item);
                println!("Productor produjo: {item}");
                item += 1;

                // Despertar a un consumidor
                cond.notify_one();

                thread::sleep(Duration::from_millis(300));
            }
        });
    }

    // CONSUMIDOR
    {
        let m = monitor.clone();
        thread::spawn(move || {
            loop {
                let (lock, cond) = &*m;
                let mut estado = lock.lock().unwrap();

                // Esperar si el buffer está vacío
                while estado.buffer.is_empty() {
                    estado = cond.wait(estado).unwrap();
                }

                // Consumir
                let item = estado.buffer.remove(0);
                println!("Consumidor consumió: {item}");

                // Despertar al productor
                cond.notify_one();

                thread::sleep(Duration::from_millis(500));
            }
        });
    }

    // Mantener vivo el main
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

struct Estado {
    buffer: Vec<i32>,
    capacidad: usize,
}
