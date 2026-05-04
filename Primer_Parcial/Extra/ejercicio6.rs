use std::sync::{Mutex, Condvar, Arc};
use std::thread;
use std::time::Duration;

struct Estado {
    esperando_aterrizaje: usize,
    esperando_despegue: usize,
    pista_libre: bool,
}

struct Aeropuerto {
    estado: Mutex<Estado>,
    cond: Condvar,
}


impl Aeropuerto{

    fn pedir_aterrizaje(&self){
        let mut st = self.estado.lock().unwrap();
        st.esperando_aterrizaje+=1; //necesito dereferencias porque es una referencia directa
        while !st.pista_libre{
            st = self.cond.wait(st).unwrap();
        }
        st.esperando_aterrizaje-=1;
        st.pista_libre = false;
    }
    
    fn terminar_aterrizaje(&self){
        let mut st = self.estado.lock().unwrap();
        st.pista_libre = true;
        drop(st);
        self.cond.notify_all();
    }
    
    fn pedir_despegue(&self){
        let mut st = self.estado.lock().unwrap();
        st.esperando_despegue += 1;
        while !st.pista_libre || st.esperando_aterrizaje >0 {
            st = self.cond.wait(st).unwrap();
        }
        st.esperando_despegue -= 1;
        st.pista_libre = false;
    }
    
    fn terminar_despegue(&self){
        let mut st = self.estado.lock().unwrap();
        st.pista_libre = true;
        drop(st);
        self.cond.notify_all();
    }
    
}

fn main(){
    let control = Arc::new(Aeropuerto {estado: Mutex::new(Estado {esperando_aterrizaje: 0, esperando_despegue:0, pista_libre:true,}), cond: Condvar::new()});
    for i in 0..10{
        if i%2==0{
            let c = control.clone();
            thread::spawn(move ||{
                c.pedir_aterrizaje();
                thread::sleep(Duration::from_secs(1));
                c.terminar_aterrizaje();
            });
        } else {
            let c = control.clone();
            thread::spawn(move ||{
                c.pedir_despegue();
                thread::sleep(Duration::from_secs(1));
                c.terminar_despegue();
                });
        }
    }
    }
        
