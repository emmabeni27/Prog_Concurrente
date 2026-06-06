class Queue<E> {
    
    //en la versión lock free simepre hay un nodo dummy. Permite que funcione sin caos especiales.  
    private class Node<E>(val item: E, var next: Node<E>?)

    private var head = AtomicReference<Node<E>?> (null)
    private var tail = AtomicReference<Node<E>?> (null)

    fun enqueue(item: E) {
        val newNode = Node(item, null)
        
        while (true) {
            val t = tail.get()
            val next = t.next.get() //es un valor leído, no un Atomic reference. von el get deja de ser atómico
            
            if (next == null){ //t realmetne es el último nodo
                if (t.next.compareAndSet(null, newNode)){ //si sigue sinedo null, enlazalo a newNode. Si lo logra...
                    tail.compareAndSet(t, newNode) //avanzo el tail
                    return
                }
                else{
                    tail.compareAndSet(t, next) //el puntero estaba atradaso
                }
            }
        }
    }

    fun dequeue_sinDummy(): E? {
        while(true){
            
        val oldHead = head.get() ?: return null
        val newHead = oldHead.next
        
        if(head.compareAndSet(oldHead, newHead)){
            return oldHead.item
        }
    }
}
