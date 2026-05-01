use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main(){

    let runway = Arc::new(Mutex::new(true));
    
    let plane1 = Arc::clone(&runway); 
    let plane2 = Arc::clone(&runway);
    
    
    //no hago clone de plane 1 y 2. Son solo referencias a runway que es un Arc. Runway no lo paso a ningún lado, por lo que osbrevive a los threads
    
    
    let landing1 = thread::spawn(move|| {
    {
        let mut val = plane1.lock().unwrap();
        *val = false;
        thread::sleep(Duration::from_secs(5));
        *val = true;
        }//libera el lock al terminar el bloque
    });  //el hilo termina acá
    
    println!("{}", runway.lock().unwrap());
    //landing1.join().unwrap();
    
    let landing2 = thread::spawn(move||{
    {
        let mut val =plane2.lock().unwrap();
        *val =false;
        }
    });
    
    //landing2.join().unwrap(); ojo, con este orden de spawn/landing no sería concurrente!
    
    landing1.join().unwrap(); //con join main espera a que temrinen los hilos hijos, si no pod´ria acabar antes y nunca se ejecuta el thread
    landing2.join().unwrap();
    
    println!("{}", runway.lock().unwrap());
    
}
