use std::sync::{Arc, RwLock};
use std::thread;

//biblioteca

fn main() {

    let biblioteca = Arc::new(RwLock::new(String::from("La Bruja")));
    let mut handles = vec![];
    
    for i in 1..=10{
        
        if i % 2 == 0 {
            let bibliotecario = biblioteca.clone();
            let h = thread::spawn(move ||{
                let mut val = bibliotecario.write().unwrap();
                *val = String::from("Grandes Esperanzas");
                println!("{}", *val);
            });
            handles.push(h);
        }
        
        if i %2 == 1{
            let lector = biblioteca.clone();
            let h = thread::spawn(move || {
                let val = lector.read().unwrap();
                println!("{}", *val);
            });
            handles.push(h);
        }
    }
    for h in handles {
        h.join().unwrap(); // si uso join no necesito sleep
    }
}

