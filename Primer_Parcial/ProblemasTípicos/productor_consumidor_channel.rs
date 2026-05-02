use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Canal: muchos productores → un consumidor
    let (tx, rx) = mpsc::channel::<i32>();

    // PRODUCTOR
    {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut item = 0;
            loop {
                println!("Productor produce: {item}");
                tx.send(item).unwrap();
                item += 1;
                thread::sleep(Duration::from_millis(300));
            }
        });
    }

    // CONSUMIDOR
    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(item) => {
                    println!("Consumidor recibe: {item}");
                    thread::sleep(Duration::from_millis(500));
                }
                Err(_) => {
                    println!("Canal cerrado");
                    break;
                }
            }
        }
    });

    // Mantener vivo el main
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
