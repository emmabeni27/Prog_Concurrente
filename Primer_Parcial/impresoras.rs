use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

struct Estado {
    pendientes: VecDeque<i32>,
    estado: bool, 
}

fn main() {
    let monitor = Arc::new((
        Mutex::new(Estado {pendientes: VecDeque::new(), estado: true,}), Condvar::new(),));

    for i in 1..=3 {
        let m = monitor.clone();
        thread::spawn(move || {
            println!("El hilo {i} quiere imprimir");

            let (mutex, cond) = &*m;
            let mut estado = mutex.lock().unwrap();

            // Si la impresora está ocupada, me pongo en la cola
            if !estado.estado {
                estado.pendientes.push_back(i);

                while !estado.estado || *estado.pendientes.front().unwrap() != i { //ehile pq vuelve a verificar, capaz lo espiertan pero sigue sin cumplir
                    estado = cond.wait(estado).unwrap(); //al entrear en espera libera el recurso
                    //aca se duerme este hilo
                }

                estado.pendientes.pop_front();
            }

            // Tomo la impresora
            estado.estado = false;

            //LIBERO EL LOCK ANTES DE IMPRIMIR, los otros de todos modos no pueden imprir si no libero el lock
            //pero si bloqueo el monitor y les devuelve flase, entran a la cola
            drop(estado);

            println!("{i} imprimiendo");
            thread::sleep(Duration::from_secs(1));

            let (mutex, cond) = &*m;
            let mut estado = mutex.lock().unwrap();
            estado.estado = true;

            cond.notify_one();
        });
    }

    thread::sleep(Duration::from_secs(5));
}
