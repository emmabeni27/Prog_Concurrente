use std::sync::{Mutex, Condvar, Arc};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

struct my_buffer{
    estado: Mutex<Estado>,
    cond: Condvar,
}

struct Estado{
    buffer: VecDeque<i32>,
    capacidad: usize,
    esperando_prioritarios: usize,
}

impl my_buffer{
    fn producir(&self, item: i32){
        let mut st = self.estado.lock().unwrap();
        while st.buffer.len() == st.capacidad {
            st = self.cond.wait(st).unwrap();
        }
        st.buffer.push_back(item);
        drop(st); //CUIDADO, no es drop de estado
        self.cond.notify_all();
    }
    
    fn consumir_normal(&self){
        let mut st = self.estado.lock().unwrap();
        while st.buffer.is_empty() || st.esperando_prioritarios > 0 {
            st = self.cond.wait(st).unwrap();
        }
        st.buffer.pop_front();
        drop(st);
        self.cond.notify_all();
    }
    
    fn consumir_prioritario(&self){
        let mut st = self.estado.lock().unwrap();
        st.esperando_prioritarios += 1;
        while st.buffer.is_empty(){
            st = self.cond.wait(st).unwrap();
        }
        st.buffer.pop_front();
        st.esperando_prioritarios -= 1;
        drop(st);
        self.cond.notify_all();
    }
}


fn main(){
    let monitor = Arc::new(my_buffer {
    estado: Mutex::new(Estado {
        buffer: VecDeque::new(),
        capacidad: 5,
        esperando_prioritarios: 0,
    }),
    cond: Condvar::new(),
});
//lanzar threads, ver si llamo a productor o cual se los consumidores
}

//cuidado: no asegura fairness entre los prioritarios, puede generar starvarion de los comunes
//agregar un max_prioritarios
