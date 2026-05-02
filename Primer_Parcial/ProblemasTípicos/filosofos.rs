use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main(){
    
    let n = 5;

    let mut tenedores = Vec::new();
    
    //modelar tenedores (recurso compartido)
    for _ in 0..n {
        let tenedor = Arc::new(Mutex::new(()));
        tenedores.push(tenedor);
    }
    
    for i in 0..n {
        //recursos asociados
        let izq = tenedores[i].clone();
        let der = tenedores[(i+1)%n].clone();
        
        thread::spawn(move || {
            {
                pensar(i); //invierto uno para que no llegue a deadlock
                if i == n-1 {
                    let _d = der.lock().unwrap();
                    let _i = izq.lock().unwrap();
                    comer(i);
                } else {
                    let _i = izq.lock().unwrap();
                    let _d = der.lock().unwrap();
                    comer(i);
                }
            }
        });
    }
    
    loop {
        thread::sleep(Duration::from_secs(1));
    }
    
}

fn pensar(id: usize){
    println!("Filósofo {id} está pensndo");
    thread::sleep(Duration::from_millis(200));
}

fn comer(id: usize){
    println!("Filósofo {id} está comiendo");
    thread::sleep(Duration::from_millis(200));
}
