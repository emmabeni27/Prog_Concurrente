//lock
use std::sync::Mutex;
let lock = Mutex::new(0); //el contenido es el dato compartido
{
    let mut val = lock.lock().unwrap();
    *val += 1;
}

//un mutex por sí solo no puede estar compartido entre threads,
// para eso tiene que tener un Arc

use std::sync::{Arc, Mutex};
use std::thread;

let runway: bool = true;
let plane1 = Arc::new(Mutex::new(runway));

let plane2 = Arc::clone(&plane1); //hago clone poruqe el move pasa ownership y no la devuelve al finalizar

// & apunta a un valor sin movimiento
// * desreferencia. 
// let x = 10;
//let r = &x;
//println!("{}", *r);

thread::spawn(move|| {
{
    let mut val = plane2.lock().unwrap();
    *val = false;
    }//libera el lock al terminar el bloque
});  //el hilo termina acá

