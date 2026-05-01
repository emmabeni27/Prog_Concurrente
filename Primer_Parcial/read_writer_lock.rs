use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main(){
    
    let greeting = Arc::new(RwLock::new(String::from("hello"))); //inicializar el rw lock!!
    
    let french = Arc::clone(&greeting);
    let german = Arc::clone(&greeting);
    let romanian1 = Arc::clone(&greeting);
    let romanian2 = Arc::clone(&greeting);
    
    //LECTOR 1
    let french = thread::spawn(move || {
        {
            let val = french.read().unwrap();
            println!("{}", *val);
            thread::sleep(Duration::from_millis(500));
        }
    });
    
    //LECTOR 2 
    let romanian1 = thread::spawn(move || {
        {
            let val = romanian1.read().unwrap();
            println!("{}", *val);
        }
    });
    
    //ESCRITOR
    let german = thread::spawn(move || {
        {
            let mut val = german.write().unwrap(); //se bloquea hasta que no haya lectores
            //mut va en la línea 34 porque el único objeto que te da acceso mutable al dato es 
            //el guard (RwLockWriteGuard<T>), no el Arc ni el RwLock.
            *val = String::from("hallo");
            println!("{}", *val);
        }
    });
    
    //LECTOR 3, no uar otra vez romanian pq se meuve al hilo y la destruye
    let romanian2 = thread::spawn(move || {
        {
            let val = romanian2.read().unwrap();
            println!("{}", *val);
        }
    });
    
    french.join().unwrap();
    german.join().unwrap();
    romanian1.join().unwrap();
    romanian2.join().unwrap();
}
