struct Queue<T>{
    content: Vec<Node<T>>,
    //índices que apuntan a posiciones dentro de vector
    head: Option<usize>, //puntero lógico, índice del primero nodeo de Vec<Node>
    tail: Option<usize>, //puntero lógico, índice último nodo
}

struct Node<T>{
    content: Option<T>,
    next: Option<usize>,
}

impl<T> Queue<T>{

    fn new() -> Self {
        Queue{
            content: Vec::new(),
            head: None,
            tail: None,
        }
    }

    fn is_empty(&self) -> bool{
        return self.head.is_none()
    }

    //no tendría límite
    fn enqueue(&mut self, value: T){
        let new_index = self.content.len();
        let new_node = Node{
            content: Some(value),
            next: None,
        };
        self.content.push(new_node);
        match self.tail {
            //queue vacía. head,tail = None y content = []
            None => {
                self.head = Some(new_index); //agrego el primer nodo en el índice 0, Some(0)
                self.tail = Some(new_index); //pero esa única posición tmb va a ser tail
            }
            Some(tail_index) => { //si no está vacía
                self.content[tail_index].next = Some(new_index); //enlazo el último nodo con el neuvo
                self.tail = Some(new_index); //muevo puntero tail al nuevo nodo
            }
        }
    }

    fn dequeue(&mut self) -> Option<T>{
        let head_index = self.head?;
        let next_index = self.content[head_index].next; // índice del nodo sgte, pero a fin de cuentas, es un núemro
        self.head = next_index;
        if next_index.is_none(){
            self.tail = None;
        }
        self.content[head_index].content.take()
    }
}