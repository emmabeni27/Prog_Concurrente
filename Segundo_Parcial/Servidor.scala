object Servidor {
    case class Registrar(nombre: String, ref: ActorRef) //registra con nombre y dirección
    case class Enviar(de: String, destino: String, texto: String)
    case class Desconectar(nombre: String)
}

object Usuario{
    case class Conectar(nombre:String, servidor: ActorRef)
    case class Mandar(destino: String, texto: String)
    case class Recibir(de: String, texto: String)
    case object Desconectarse
}

class Servidor extends Actor{
    
    import Servidor._
    
    def receive: Receive = gestionar(Map.empty) //Mi comportamiento inicial es la función gestionar, usando un mapa vacío de usuarios
    
    def gestionar(usuarios: Map[String, ActorRef]): Receive ={
        case Registrar(nombre, ref) => context.become(gestrionar(usuarios+ (nombre -> ref))) // es lo mismo que (nombre, ref)
        
        case Enviar(de, destino, texto) => usuarios.get(destio) match => 
            case Some(ref) => ref ! Usuario.Recibir(de, texto)
            case None => sender() ! "Usuario no encontrado"
            
        case Desconectar(nombre) => 
            context.become(gestionar(usuarios-nombre))
    }
}

class Usuario extends Actor{
    
    import Usuario._
    
    def receive: Receive = desconectado
    
    def desconectado: Receive={
        
        case Conectar (nombre, servidor) => //nombre  y servidor vienen dentro del mensaje
            servidor ! Servidor.Registrar(nombre, self)
            context.become(conectado(nombre, servidor))

    }
    
    def conectado(nombre:String, servidor: ActorRef): Receive = {
            
        case Mandar(destino, texto) => 
            servidor ! Servidor.Enviar(nombre, destino, texto)
            
        case Recibir(de, texto) =>
            println(s"[$nombre] mensaje $de: $texto")
            
        case Desconectarse =>
            servidor ! Servidor.Desconectar(nombre)
            context.become(desconectado)
    }
    
}
