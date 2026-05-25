use std::ptr;

use std::sync::{
    Arc,
    Mutex,
    Condvar,
    atomic::{
        AtomicPtr,
        AtomicUsize,
        Ordering,
    },
};

use std::thread;
use std::time::Instant;

// =====================================================
// QUEUE BLOQUEANTE
// =====================================================

struct Queue<T> {
    content: Vec<Node<T>>,

    // índices dentro del vector
    head: Option<usize>,
    tail: Option<usize>,
}

struct Node<T> {
    content: Option<T>,
    next: Option<usize>,
}

struct QueueB<T> {
    components: Mutex<Queue<T>>,
    cond: Condvar,
}

impl<T> QueueB<T> {

    fn new() -> Self {

        QueueB {

            components: Mutex::new(
                Queue {
                    content: Vec::new(),
                    head: None,
                    tail: None,
                }
            ),

            cond: Condvar::new(),
        }
    }

    fn is_empty(&self) -> bool {

        let components =
            self.components.lock().unwrap();

        components.head.is_none()
    }

    fn enqueue(&self, value: T) {

        let mut components =
            self.components.lock().unwrap();

        let new_index =
            components.content.len();

        let new_node = Node {
            content: Some(value),
            next: None,
        };

        components.content.push(new_node);

        match components.tail {

            // cola vacía
            None => {

                components.head =
                    Some(new_index);

                components.tail =
                    Some(new_index);
            }

            // cola no vacía
            Some(tail_index) => {

                components.content[tail_index]
                    .next = Some(new_index);

                components.tail =
                    Some(new_index);
            }
        }

        // despierta consumidores
        self.cond.notify_one();
    }

    fn dequeue(&self) -> T {

        let mut components =
            self.components.lock().unwrap();

        // esperar mientras esté vacía
        while components.head.is_none() {

            components =
                self.cond.wait(components).unwrap();
        }

        let head_index =
            components.head.unwrap();

        let next_index =
            components.content[head_index].next;

        components.head = next_index;

        // quedó vacía
        if next_index.is_none() {
            components.tail = None;
        }

        components.content[head_index]
            .content
            .take()
            .unwrap()
    }
}

// =====================================================
// LOCK FREE QUEUE
// =====================================================

struct LFNode<T> {

    // dummy => None
    // reales => Some
    value: Option<T>,

    next: AtomicPtr<LFNode<T>>,
}

pub struct LockFreeQueue<T> {

    // apunta al dummy
    head: AtomicPtr<LFNode<T>>,

    // apunta al último nodo real
    tail: AtomicPtr<LFNode<T>>,
}

impl<T> LockFreeQueue<T> {

    pub fn new() -> Self {

        let dummy =
            Box::into_raw(
                Box::new(
                    LFNode {

                        value: None,

                        next: AtomicPtr::new(
                            ptr::null_mut()
                        ),
                    }
                )
            );

        Self {

            head: AtomicPtr::new(dummy),

            tail: AtomicPtr::new(dummy),
        }
    }

    pub fn enqueue(&self, value: T) {

        let new_node =
            Box::into_raw(
                Box::new(
                    LFNode {

                        value: Some(value),

                        next: AtomicPtr::new(
                            ptr::null_mut()
                        ),
                    }
                )
            );

        loop {

            let tail =
                self.tail.load(Ordering::Acquire);

            let next = unsafe {
                (*tail).next.load(Ordering::Acquire)
            };

            // validar tail
            if tail ==
                self.tail.load(Ordering::Acquire)
            {

                // tail real
                if next.is_null() {

                    let res = unsafe {

                        (*tail).next.compare_exchange(
                            ptr::null_mut(),
                            new_node,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        )
                    };

                    if res.is_ok() {

                        let _ =
                            self.tail.compare_exchange(
                                tail,
                                new_node,
                                Ordering::AcqRel,
                                Ordering::Acquire,
                            );

                        return;
                    }

                } else {

                    // ayudar a mover tail
                    let _ =
                        self.tail.compare_exchange(
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

            let head =
                self.head.load(Ordering::Acquire);

            let tail =
                self.tail.load(Ordering::Acquire);

            let next = unsafe {
                (*head).next.load(Ordering::Acquire)
            };

            // validación
            if head ==
                self.head.load(Ordering::Acquire)
            {

                // vacía
                if next.is_null() {
                    return None;
                }

                // tail atrasado
                if head == tail {

                    let _ =
                        self.tail.compare_exchange(
                            tail,
                            next,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        );

                    continue;
                }

                // leer valor
                let value = unsafe {
                    (*next).value.take()
                };

                // mover head
                let res =
                    self.head.compare_exchange(
                        head,
                        next,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    );

                if res.is_ok() {

                    // simplificación académica:
                    // liberar dummy viejo

                    unsafe {
                        drop(Box::from_raw(head));
                    }

                    return value;
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {

        let head =
            self.head.load(Ordering::Acquire);

        let next = unsafe {
            (*head).next.load(Ordering::Acquire)
        };

        next.is_null()
    }
}

impl<T> Drop for LockFreeQueue<T> {

    fn drop(&mut self) {

        while self.dequeue().is_some() {}

        let dummy =
            self.head.load(Ordering::Relaxed);

        unsafe {
            drop(Box::from_raw(dummy));
        }
    }
}

// =====================================================
// BENCHMARK BLOQUEANTE
// =====================================================

fn benchmark_blocking(
    producers: usize,
    consumers: usize,
    items_per_producer: usize,
) {

    let queue =
        Arc::new(
            QueueB::<usize>::new()
        );

    let consumed =
        Arc::new(
            AtomicUsize::new(0)
        );

    let total_items =
        producers * items_per_producer;

    let start = Instant::now();

    let mut handles = vec![];

    // =================================================
    // PRODUCTORES
    // =================================================

    for p in 0..producers {

        let q = Arc::clone(&queue);

        let handle =
            thread::spawn(move || {

                for i in 0..items_per_producer {

                    q.enqueue(
                        p * items_per_producer + i
                    );
                }
            });

        handles.push(handle);
    }

    // =================================================
    // CONSUMIDORES
    // =================================================

    for _ in 0..consumers {

        let q = Arc::clone(&queue);

        let consumed_counter =
            Arc::clone(&consumed);

        let handle =
            thread::spawn(move || {

                loop {

                    let current =
                        consumed_counter.load(
                            Ordering::Relaxed
                        );

                    if current >= total_items {
                        break;
                    }

                    q.dequeue();

                    consumed_counter.fetch_add(
                        1,
                        Ordering::Relaxed,
                    );
                }
            });

        handles.push(handle);
    }

    // =================================================
    // JOIN
    // =================================================

    for h in handles {
        h.join().unwrap();
    }

    let elapsed = start.elapsed();

    println!("==============================");
    println!("BLOCKING QUEUE");
    println!("Inserted: {}", total_items);

    println!(
        "Consumed: {}",
        consumed.load(Ordering::Relaxed)
    );

    println!("Time: {:?}", elapsed);
    println!("==============================");
}

// =====================================================
// BENCHMARK LOCK FREE
// =====================================================

fn benchmark_lockfree(
    producers: usize,
    consumers: usize,
    items_per_producer: usize,
) {

    let queue =
        Arc::new(
            LockFreeQueue::<usize>::new()
        );

    let consumed =
        Arc::new(
            AtomicUsize::new(0)
        );

    let total_items =
        producers * items_per_producer;

    let start = Instant::now();

    let mut handles = vec![];

    // =================================================
    // PRODUCTORES
    // =================================================

    for p in 0..producers {

        let q = Arc::clone(&queue);

        let handle =
            thread::spawn(move || {

                for i in 0..items_per_producer {

                    q.enqueue(
                        p * items_per_producer + i
                    );
                }
            });

        handles.push(handle);
    }

    // =================================================
    // CONSUMIDORES
    // =================================================

    for _ in 0..consumers {

        let q = Arc::clone(&queue);

        let consumed_counter =
            Arc::clone(&consumed);

        let handle =
            thread::spawn(move || {

                loop {

                    let current =
                        consumed_counter.load(
                            Ordering::Relaxed
                        );

                    if current >= total_items {
                        break;
                    }

                    if q.dequeue().is_some() {

                        consumed_counter.fetch_add(
                            1,
                            Ordering::Relaxed,
                        );
                    }
                }
            });

        handles.push(handle);
    }

    // =================================================
    // JOIN
    // =================================================

    for h in handles {
        h.join().unwrap();
    }

    let elapsed = start.elapsed();

    println!("==============================");
    println!("LOCK FREE QUEUE");
    println!("Inserted: {}", total_items);

    println!(
        "Consumed: {}",
        consumed.load(Ordering::Relaxed)
    );

    println!("Time: {:?}", elapsed);
    println!("==============================");
}

// =====================================================
// MAIN
// =====================================================

fn main() {

    let producers = 4;

    let consumers = 4;

    let items = 100000;

    benchmark_blocking(
        producers,
        consumers,
        items,
    );

    benchmark_lockfree(
        producers,
        consumers,
        items,
    );
}