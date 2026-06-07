data class State<A, B>(val first: A, val second: B)

class AtomicPair<A, B>(initialFirst: A, initialSecond: B) {
    
    private val state: State = AtomicReference(State(initialFirst, initialSecond))

    fun getFirst(): A{
        return state.get().first
    }

    fun getSecond(): B{
        return state.get().second
    }

    fun set(newFirst: A, newSecond: B) {
        state.set(State(newFirst, newSecond))
    }

    fun swap(): Pair<A, B> {
        while(true){
        
            val oldState = state
            val newState = State(oldState.second, oldState.first)
            if(state.compareAndSet(oldState, newState)) {
                return Pair(oldState.first, oldState.second) //devuelvo como estaba antes de intercambiar
            }
        }
    }

    fun compareAndSwap(expectedFirst: A, expectedSecond: B, newFirst: A, newSecond: B): Boolean {
        //si el estado actual es exactamente el que espero, actualizalo al nuevo. Si no, no hagas nada
        val expectedState = State(expectedFirst, expectedSecond)
        val newState = State(newFirst, newSecond)
        return state.compareAndSet(expectedState, newState)
    }
}

// swap tiene que completarse sí o sí, reinente
// compare and Swap puede fallar, y es valioso. No reintento
