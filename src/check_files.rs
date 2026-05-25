mod queue_no_bloqueante;

fn main() {
    let q = queue_no_bloqueante::LockFreeQueue::<i32>::new();
    q.enqueue(1);
    assert_eq!(q.dequeue(), Some(1));
}
