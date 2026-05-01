use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main(){
    
    //Arc<(Mutex<i32>, Condvar)>
    let monitor = Arc::new((Mutex::new(false), Condvar::new(),));
    
    //hilo esperando
    let waiter_pair = Arc::clone(&monitor);
    let waiter = thread::spawn(move ||{
        let (lock, cvar) = &*waiter_pair; //* desreferencia el Arc, & pasa referencia del contendio de la tupla
        
        let mut value = lock.lock().unwrap();
        
        if !*value {
            println!("Still waiting...");
            value = cvar.wait(value).unwrap();
        } 
        println!("At long last!"); //se cumple y sale
    });
    
    //hilo que maneja el valor
    let setter_pair = Arc::clone(&monitor);
    let setter = thread::spawn(move ||{
        let (lock, cvar) = &*setter_pair;
        
        for i in 1..=10{ //como tiene un bucle de 10, por más que el otro salga, este sigue iternado
            thread::sleep(Duration::from_secs(2));
            
            let mut value = lock.lock().unwrap();
            *value = true;
            println!("Steady... ready... go...");
            
            cvar.notify_one();
        }
    });
    
    waiter.join().unwrap();
    setter.join().unwrap();
}
