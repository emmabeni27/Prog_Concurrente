use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

//aca ya no es opcinal usar el dummy. Y como uso summy, no mantengo el vector
//el vector sería incompatible con una estrctura lock free

struct Node<T> {
    value: Option<T>, //dummy tiene none y nodos reales some
    next: AtomicPtr<Node<T>>,//spunta al útlimo, o al dummy si es´ta vcio
}

pub struct LockFreeQueue<T> {
    head: AtomicPtr<Node<T>>, //apunta a dummy
    tail: AtomicPtr<Node<T>>, //apunta a último nodo real
}

impl<T> LockFreeQueue<T> {
    pub fn new() -> Self {
        // cro dummy y los punteros al mismo. Así inicializo queue
        let dummy = Box::into_raw(Box::new(Node {
            value: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    pub fn enqueue(&self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value: Some(value),
            next: AtomicPtr::new(ptr::null_mut()), //next es null porque es el útlimo que estoy agregando
        }));

        loop { //sigo bsucando cual es el verdadero tail. REinento leer el rail y encontrar tail.next cunado sea null.
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };

            // ¿tail sigue siendo válido?
            if tail == self.tail.load(Ordering::Acquire) {

                // tail realmente es el último nodo
                if next.is_null() {

                    // Intentar enlazar el nuevo nodo
                    let res = unsafe {
                        (*tail).next.compare_exchange(
                            ptr::null_mut(),
                            new_node,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        )
                    };

                    if res.is_ok() {
                        // Intentar mover tail
                        let _ = self.tail.compare_exchange(
                            tail,
                            new_node,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        );

                        return;
                    }

                } else {
                    // Ayudar a otro thread a mover tail
                    let _ = self.tail.compare_exchange(
                        tail,
                        next,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    );
                }
            }
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);

            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            // Validación
            if head == self.head.load(Ordering::Acquire) {

                // Cola vacía
                if next.is_null() {
                    return None;
                }

                // tail atrasado
                if head == tail {
                    let _ = self.tail.compare_exchange(
                        tail,
                        next,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    );

                    continue;
                }

                // Leer valor antes del CAS
                let value = unsafe {
                    (*next).value.take()
                };

                // Intentar mover head
                let res = self.head.compare_exchange(
                    head,
                    next,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );

                if res.is_ok() {

                    // Liberar viejo dummy
                    unsafe {
                        drop(Box::from_raw(head));
                    }

                    return value;
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        let head = self.head.load(Ordering::Acquire);

        let next = unsafe {
            (*head).next.load(Ordering::Acquire)
        };

        next.is_null()
    }
}

impl<T> Drop for LockFreeQueue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}

        let dummy = self.head.load(Ordering::Relaxed);

        unsafe {
            drop(Box::from_raw(dummy));
        }
    }
}