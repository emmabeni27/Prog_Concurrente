use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

fn main(){
    
    let bath = Arc::new(Mutex::new(true));
    
    let mut handles =vec![];
    
    for i in 0..5 {
        let bath =bath.clone();
        
        handles.push(thread::spawn(move || {
            println!("Persona {i} quiere entrar al baño");
            
            let _lock = bath.lock().unwrap();
            println!("Pesona {i} usando el baño");
            
            thread::sleep(Duration::from_millis(500));
            println!("Persona {i} libera el baño"); //al salir de scope libera el recurso
        }));
    }
    for h in handles{
        h.join().unwrap();
    }
}
