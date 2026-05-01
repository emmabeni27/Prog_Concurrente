use std::sync::{Arc, Mutex};
use std::thread;
//use std::time::Duration;

fn main(){
    let runway = true;
    let plane1 = Arc::new(Mutex::new(runway)); //vive en hilo main. Puedo suar pra leer o modificar pista cuando hijo termina
    println!("{}", *plane1.lock().unwrap()); //para printear tengo que lockear (en el fondo es un read de la var). S elibera solo por scope
    
    let plane1_clone = Arc::clone(&plane1); //hago clone poruqe el move pasa ownership y no la devuelve al finalizar. Así puedo modificar
    
    //plane 1 y clone son dos punteros a Arc, pero si no creo la copia clone
    // sl moverlo dentro del hilo perdería plane 1
    
    let take = thread::spawn(move|| {
    {
        let mut val = plane1_clone.lock().unwrap();
        *val = false;
        }//libera el lock al terminar el bloque
    });  //el hilo termina acá
    
    
    //thread::sleep(Duration::from_millis(50)); //si no, se ejecuta tan rpaido que imprime ntes de que termine el hilo
    
    //otra opción es guardar thread e un let y aplicar join
    
    take.join().unwrap();
    
    println!("{}", *plane1.lock().unwrap()); //LOCK ME DEVUELVE UNA REFERNCIA PROTEGIDA MutexGuard<bool>.Por eso tengo que dereferenciar
    //println!("{}", *plane1_clone.lock().unwrap()); --> muere cuando sale del scope
}
