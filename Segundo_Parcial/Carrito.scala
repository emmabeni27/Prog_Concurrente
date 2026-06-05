actor: carrito
estado: abierto, cerrado
acciones: agregar/sacar productos | listo pra pagar-no se peude modificar

object Carrito {
    case class Agregar(producto: String, precio: Double)
    case class Quitar(producto: String)
    case object Cerrar
    case object Pagar
    case object Consultar
}


class Carrito extends Actor{
    import Carrito._
    
    //receive es el estado actual
    def receive: Receive = abierto(Map.empty) //necesito el mapa inicial
    
    def abierto(items: Map[String, Double]): Receive = {
            //en los case no va object ni class
        case Agregar(producto, precio) => context.become(abierto(items + (producto -> precio)))
        case Quitar(producto) => context.become(abierto(items - producto))
        case Cerrar => context.become(cerrado(items)) //no olvidar pasar apramtros!!!! 
        case Pagar => sender() ! "Cerrar para pagar" //sumo todos los asociados
        case Consultar => sender() ! items //el asociado a esta invoacación
    }
    
    def cerrado(items: Map[String, Double]): Receive ={
        case Agregar => sender() ! "Carrito cerrado. No se puede agregar producto."
        case Quitar => sender() ! "Carrito cerrado. No se puede quitar producto."
        case Pagar => sender() ! items.values.sum
        case Consultar => sender() ! items
    }
}
