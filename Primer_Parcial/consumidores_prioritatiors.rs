use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

fn main(){
    
    let capacidad: usize = 10;
    
    let monitor = Arc::new((
    Mutex::new(Situacion {buffer: VecDeque::new(), productores: VecDeque::new(), consumidores: VecDeque::new(), 
    consumidores_prioritarios: VecDeque::new(),}), Condvar::new(),));
    
    for i in 1..=10{
        
        //productor
        if i % 3 == 1  {
            let m = monitor.clone();
            let c = capacidad.clone();
        
            thread::spawn(move || {
                println!("El hilo {i} quiere producir");
                
                let (mutex, cond)= *&m;
                let mut situacion = mutex.lock().unwrap();
                
                if situacion.buffer.len() == c {
                    situacion.productores.push_back(i);
                    
                    while situacion.buffer.len() == c{
                        situacion = cond.wait(situacion).unwrap();
                    }
                    
                    situacion.productores.pop_front();
                } 
                
                //cuando wait retorna ya tengo el lock de nuevo
                prinln!("El hilo {i} está escribiendo");
                situacion.buffer.push_back(i);
                drop(lock);
                cond.notify_all();
            });
        }
        
        //consumidor
        if i % 3 == 2{
            let m = monitor.clone();
            let c = capacidad.clone();
            
            thread::spawn(move ||
            println!("EL hilo {i} quiere consumir");
            
            let (mutex, cond) = *&m;
            let mut situacion = mutex.lock().unwrap();
            
            if situacion.buffer.is_empty() || !situacion.consumidores_prioritarios.is_empty(){
                situacion.consumidores.push_back(i);
                
                while situacion.buffer.is_empty() || !situacion.consumidores_prioritarios.is_empty(){
                    situacion = cond.wait(situacion).unwrap();
                }
                situacion.consumidores.pop_front();
            }
            println!("El hilo {i} está consumienod");
            situacion.buffer.pop_front();
            drop(lock);
            cond.notify_all();
            )
        }
        
        //consumidor prioritario
        if i % 3 == 0 {
        
            let m = monitor.clone();
            let c = capacidad.clone();
            
            threads::spawn(move ||{
                println!("El hilo {i} NECESITA consumir");
                
                let (mutex, cond) = &*m;
                let mut situacion = mutex.lock().unwrap();
                
                if situacion.buffer.is_empty(){
                    situacion.consumidores_prioritarios.push_back(i);
                    
                    while situacion.buffer.is_empty(){
                        situacion = cond.wait(situacion).unwrap();
                    }
                    
                    situacion.consumidores_prioritarios.pop_front();
                }
                prinln!("EL hilo prioritario {i} va a consumir");
                buffer.pop_front();
                drop(lock);
                cond.notify_all();
            })
        }
    }
    
    
}

struct Situacion{
    buffer: VecDeque<T>,
    productores: VecDeque<T>
    consumidores: VecDeque<T>,
    consumidores_prioritarios: VecDeque<T>,
}

------------------------------------------------------------------------------------------------------------------------------------------------------------
VERSIÓN CORREGIDA

use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

struct Situacion {
    buffer: VecDeque<i32>,
    waiting_prioritarios: usize, //NO NECESITO VECTOR DE LOS QUE ESPERAN SI USO CONDVAR
}

fn main() {
    let capacidad: usize = 5; //implement copy, no necesito CLONARLO

    let monitor = Arc::new((
        Mutex::new(Situacion {
            buffer: VecDeque::new(),
            waiting_prioritarios: 0,
        }),
        Condvar::new(),
    ));

    for i in 1..=10 {
        let m = monitor.clone();

        // 🟢 PRODUCTOR
        if i % 3 == 1 {
            thread::spawn(move || {
                println!("Prod {i} quiere producir");

                let (mutex, cond) = &*m;
                let mut s = mutex.lock().unwrap();

                while s.buffer.len() == capacidad {
                    s = cond.wait(s).unwrap();
                }

                println!("Prod {i} produce");
                s.buffer.push_back(i);

                drop(s);
                cond.notify_all();
            });
        }

        // 🔵 CONSUMIDOR NORMAL
        if i % 3 == 2 {
            thread::spawn(move || {
                println!("Cons {i} quiere consumir");

                let (mutex, cond) = &*m;
                let mut s = mutex.lock().unwrap();

                while s.buffer.is_empty() || s.waiting_prioritarios > 0 {
                    s = cond.wait(s).unwrap();
                }

                let val = s.buffer.pop_front().unwrap();
                println!("Cons {i} consume {val}");

                drop(s);
                cond.notify_all();
            });
        }

        // 🔴 CONSUMIDOR PRIORITARIO
        if i % 3 == 0 {
            thread::spawn(move || {
                println!("PRIO {i} quiere consumir");

                let (mutex, cond) = &*m;
                let mut s = mutex.lock().unwrap();

                s.waiting_prioritarios += 1;

                while s.buffer.is_empty() {
                    s = cond.wait(s).unwrap();
                }

                s.waiting_prioritarios -= 1;

                let val = s.buffer.pop_front().unwrap();
                println!("PRIO {i} consume {val}");

                drop(s);
                cond.notify_all();
            });
        }
    }

    thread::sleep(Duration::from_secs(3));
}
