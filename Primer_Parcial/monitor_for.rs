use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    // Monitor: estado + condvar
    let pair = Arc::new((
        Mutex::new(0),      // estado: un entero cualquiera
        Condvar::new(),     // condición: esperar hasta que llegue a 5
    ));

    // Hilo que espera
    let waiter_pair = Arc::clone(&pair);
    let waiter = thread::spawn(move || {
        let (lock, cvar) = &*waiter_pair;

        let mut value = lock.lock().unwrap();

        while *value < 5 {
            println!("Esperando... valor actual = {}", *value);
            value = cvar.wait(value).unwrap(); // libera el lock y duerme
        }

        println!("Listo! El valor llegó a {}", *value);
    });

    // Hilo que incrementa el valor
    let setter_pair = Arc::clone(&pair);
    let setter = thread::spawn(move || {
        let (lock, cvar) = &*setter_pair;

        for i in 1..=5 {
            thread::sleep(Duration::from_secs(1));

            let mut value = lock.lock().unwrap();
            *value = i;
            println!("Setter: valor actualizado a {}", i);

            cvar.notify_one(); // despierta al que espera
        }
    });

    waiter.join().unwrap();
    setter.join().unwrap();
}
