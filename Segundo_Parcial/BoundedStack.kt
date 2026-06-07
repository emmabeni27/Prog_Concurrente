class BoundedStack<E>(val capacity: Int) {

    class Node<E>(val item: E, val next: Node<E>? = null) //esto queda así
    
    private var top = AtomicReference<Node<E>?>(null)
    private var size = AtomicInteger(0)

    fun push(item: E): Boolean {
        
        //lo preparo directamente abajo, en el newTop del while, no lo necesito acá
        //val newNode = Node(item, top) //preparo nodo
        
        while(true){
            
            if (size.get() >= capacity) return false //chequeo capacidad
        
            val oldTop = top.get()
            val newTop = Node(item, oldTop)
            
            if(top.compareAndSet(oldTop, newTop)){
                size.getAndIncremet()
                return true
            } 
        }
        
    }

    fun pop(): E? {
        
        while(true){
            
            if(size.get() == 0)  return null
        
            val oldTop = top.get()
            val newTop = oldTop.next
            
            if(top.compareAndSet(oldTop, newTop)){
                size.getAndDecrement()
                return oldTop.item
            }
            
        }
    }
}
