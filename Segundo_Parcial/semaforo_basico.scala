object Semaforo {
    //si no tiene parametros es object en lugar de lcass
    case object Avanzar //responde al sender()
    case object CambiarLuz //es cíclico
    case object ConcultarEstado //responde al sender()
}

class Semaforo extends Actor {
    import Semaforo._
    
    private var estado = "Rojo"
    
    def receive: Receive = {
        
        case Avanzar =>
            if(estado == "Rojo" || estado == "Amarillo"){
                sender() ! "Denegado"
            }
            else{
                sender() ! "Permitido"
                
            }
        
        case CambiarLuz =>
            if (estado == "Rojo"){
                estado = "Verde"
            }
            else if (estado == "Verde"){
                estado = "Amarillo"
            }
            else (estado == "Amarillo"){
                estado = "Rojo"
            }
        case ConsultarEstado => 
        sender() ! estado
    }
}

//es correcto, pero el actor cambia un dato interno en lugar de cambiar su comportamiento como haría con become. 
