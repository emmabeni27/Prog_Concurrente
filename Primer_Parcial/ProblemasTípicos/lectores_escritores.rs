use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    
    let dato = Arc::new(RwLock::new(0));
    
    for i in 0..3{
        let d = dato.clone();
        thread::spawn(move || {
            loop {
                let valor = d.read().unwrap();
                println!("Lector {i} lee: {}", *valor);
                thread::sleep(Duration::from_millis(300));
            }
        });
    }
    
    let escritor = dato.clone();
    thread::spawn(move || {
        loop{
            let mut valor = escritor.write().unwrap();
            *valor+=1;
            println!("Escritor actualiza a {}", *valor);
        }
        thread::sleep(Duration::from_secs(1));
    });
    
    loop {thread::sleep(Duration::from_secs(1));}
    
}
