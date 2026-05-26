use std::sync::{Mutex, Condvar, Arc};

struct Queue<T>{
    content: Vec<Node<T>>,
    //índices que apuntan a posiciones dentro de vector
    head: Option<usize>, //puntero lógico, índice del primero nodeo de Vec<Node>
    tail: Option<usize>, //puntero lógico, índice último nodo
}

struct QueueB<T>{
    components: Mutex<Queue<T>>, //mutex solo puede tener un parametro
    cond: Condvar
}

struct Node<T>{
    content: Option<T>,
    next: Option<usize>,
}

impl<T> QueueB<T>{

    fn new() -> Self { //devuelo algo nuevo
        return QueueB{
            components: Mutex::new(Queue{content: Vec::new(), head: None, tail: None,}), //el arc no va aca
            cond: Condvar::new()
        }; //no olvidar anotar el return!!!
    }

    fn is_empty(&self) -> bool{ //bloque y veo si es none head
        let components = self.components.lock().unwrap(); //si no puede adquirir el lock, se bloquea y desbloqueará solo caundo el mutex lo libere. Mientras tanto, no consume recursos. No lo libera pq nunca lo tomó
        return components.head.is_none() //puedo omitir el return. Al salir de scope dropea automático
        //no uso notify, eso es para caso manual y aca se encarga el SO de despertarlos
        //no uso condvar pq no hay otra condición a la que esperar
    }

    //no tendría límite
    fn enqueue(&self, value: T){ //lock, nuevo índice, creo nodo con contenido nuevo, pusheo nuevo nodo, reacomodo ectores si none o some. NOtify y dropea solo
        let mut components = self.components.lock().unwrap();

        let new_index = components.content.len();
        let new_node = Node{
            content: Some(value),
            next: None,
        };
        components.content.push(new_node);
        match components.tail {
            //queue vacía. head,tail = None y content = []
            None => {
                components.head = Some(new_index); //agrego el primer nodo en el índice 0, Some(0)
                components.tail = Some(new_index); //pero esa única posición tmb va a ser tail
            }
            Some(tail_index) => { //si no está vacía
                components.content[tail_index].next = Some(new_index); //enlazo el último nodo con el neuvo
                components.tail = Some(new_index); //muevo puntero tail al nuevo nodo
            }
        }
        drop(components); //dropea solo, no hace falta pero tampoco daño. Mejor quizás nates del notify
        self.cond.notify_one(); //despierta a los que esperaban dequeue porqu estaba vacía
    }

    fn dequeue(&self) -> T {
        let mut components = self.components.lock().unwrap();
        while components.head.is_none() {
            components = self.cond.wait(components).unwrap(); //libera y despierta cuando se llene
        }

        let head_index = components.head.unwrap();
        let next_index = components.content[head_index].next;
        components.head = next_index;
        if next_index.is_none() {//si uso is_empty gnero un deadlock
            components.tail = None;
        }
        let value = components.content[head_index].content.take().unwrap();
        value
    }

}

