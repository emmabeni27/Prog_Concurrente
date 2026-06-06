class Counter {
    private val value = AtomicInteger(0)
    
    fun increment(){
        
        while(true){
            val old = value.get()
            if(value.compareAndSet(old, old + 1)) return
        }
    }
    
    fun decrement(){
        
        while(true) {
            val old = value.get()
            val new = old - 1
            if(value.compareAndSet(old, new)){
                return new
            }
        }
        
    }
    
    fun addAndGet(delta: Int): Int{
    
        while (true) {
            val old = value.get()
            val new = old + delta
            if (value.compareAndSet(old, new)){
                return new
            }
        }
        
    }
    
    fun get(): Int = value.get()
}
