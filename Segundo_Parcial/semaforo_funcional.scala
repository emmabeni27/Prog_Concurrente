object Semaforo {
    //si no tiene parametros es object en lugar de lcass
    case object Avanzar //responde al sender()
    case object CambiarLuz //es cíclico
    case object ConsultarEstado //responde al sender()
}

class Semaforo extends Actor {
    import Semaforo._
    
    def receiver: Receive = enRojo
    
    def enRojo: Receive = {
        case Avanzar => sender() ! "Denegado"
        case CambiarLuz => context.become(enVerde)
        case ConsultarEstado => sender() ! "Rojo"
    }
    
    def enAmarillo: Recieve = {
        case Avanzar => sender() ! "Denegado"
        case CambiarLuz => context.become(enRojo)
        case ConsultarEstado => sender() ! "Amarillo"
    }
    
    def enVerde: Receive = {
        case Avanzar => sender() ! "Permitido"
        case CambiarLuz => context.become(enAmarillo)
        case ConsultarEstado => sender() ! "Verde"
    }
}

//ahora es totalmente programación funcional. Paso de paraémtro el método que quiero que llame
