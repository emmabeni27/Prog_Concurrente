actores: subasta y postor
estados: subasta abierta y cerrada, postor inactivo o participando

//esqueleto de mensaje
object Subasta{
    case class Register(nombre: String, ref: ActorRef)
    case class Ofertar(nombre: String, monto: Double)
    case object Cerrar
    case object Consultar
}

object Postor{
    case class Unirse(nombre: String, subasta: ActorRef)
    case class HacerOferta(monto: Double)
    case class Notificacion(texto: String)
    case object Retirarse
}

class Postor extends Actor{
    
    import Postor._
    
    def receive: Receive = inactivo
    
    def participando(nombre: String, subasta: String): Receive = {
        case HacerOferta(monto) => subasta ! Subasta.Ofertar(nombre, monto)
        case Notificacion(texto) => println(s"[$nombre] $texto") 
        case Retirarse => context.become(inactivo)
    }
    
    def inactivo: Receive = {
        case Unirse(nombre, subasta) => 
            subasta ! Subasta.Register(nombre, self)
            context.become(participando(nombre, subasta))
    } 
}

class Subasta extends Actor{
    
    import Subasta._
    
    def receive: Receive = abierta(Map.empty, "nadie", 0.0) //inicializar todos los valoes
    
    def abierta(postores: Map[String, ActorRef], mejorNombre: String, mejorMonto: Double): Receive = {
        
        case Register(nombre, ref) => context.become(abierta(postores + (nombre -> ref), mejorNombre, mejorMonto))
        
        case Ofertar(nombre, monto) => 
            if (monto > mejorMonto){
               //no son var, no puedo mutar y reasignar
               context.become(abierta(postores, nombre, monto))
            }
            
        case Cerrar => 
            postores.values.foreach(ref => ref ! Postor.Notificacion(s"Ganador: $mejorNombre con $mejorMonto"))
            context.become(cerrada(postores, mejorNombre))
        
        case Consultar => sender() ! s"Mejor oferta: $mejorNombre con $mejorMonto"
    }
    
    def cerrada(postores: Map[String, ActorRef], mejorNombre: String): Receive = {
        case Consultar => sender() ! s"Ganador: $mejorNombre" //le responde solo a quien pregunta
        
    }
}
