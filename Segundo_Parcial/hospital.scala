object Paciente {
    case class Registrarse(nombre: String, hospital: ActorRef) //cuando se registra pide atención y pasa a esperando
    case class ConsultaAsignada(medicoNombre: String, medicoRef: ActorRef)
    case object ConsultaTerminada
}

object Medico {
    case class Atender(pacienteNombre: String, pacienteRef: ActorRef)
    case object TerminarConsulta //los datos que necesita ya los tiene cargados
}

object Hopsital {
    case class RegistrarMedico(nombre: String, ref: ActorRef)
    case class SolicitarAtencion(pacienteNombre: String, pacienteRef: ActorRef) //info que le pasa apciente cuando llma 
    case class MedicoLibre(nombre: String) //con el nombre busco la ref en el map del hospital. Ahí lo enceuntra y actualiza el estado a libre
}

class Paciente extends Actor {
    
    import Paciente._
    
    def recieve: Receive = inactivo
    
    def inactivo: Receive = {
        case Registrarse(nombre, hospital) =>
            hospital ! Hospital.SolicitarAtencion(nombre, self)
            context.become(esperando(nombre, hospital))
        case Notificación(texto: String)
    }

    def esperando(nombre: String, hospital: ActorRef): Receive = {
        case ConsultaAsignada(medicoNombre, medicoRef) =>
            context.become(enConsulta(nombre, hospital, medicoRef))
    }

    def enConsulta(nombre: String, hospital: ActorRef, medicoRef: ActorRef): Receive = {
        case ConsultaTerminada =>
            medicoRef ! Medico.TerminarConsulta
            context.become(esperando(nombre, hospital))
    }
}

class Medico extends Actor {
    
    import Medico._
    
    def recieve: Recieve = libre(nombre, hospital)
    
    def atendiendo(nombre: String, hospital: ActorRef) = {
        case TerminarConsulta => 
        hospital ! Hospital.MedicoLibre(nombre)
        context.become(libre(nombre, hospital))
    }
    
    def libre(nombre: String, hospital: ActorRef): Recieve = {
        case Atender(pacienteNombre, paceinteRef) => 
            pacienteRef ! Paciente.ConsultaAsignada(nombre, self)
            context.become(atendiendo(nombre, hospital))
    }
    
}

class Hospital extends Actor { //no cambia de comportameinto, siempre hace lo mismo
    
    import Hospital._
    
    //no tiene estado cambiante
    
    def gestion(libres: Map[String, ActorRef], ocupados: Map[String, ActorRef]): Recieve{
        case RegistrarMedico(nombre, ref) => libres + (nombre -> ref)
        case SolicitarAtencion(pacienteNombre, pacienteRef) =>
            if (libres.isEmpty) {
                pacienteRef ! Paciente.Notificacion("No hay médicos disponibles")
            } else {
                val (medicoNombre, medicoRef) = libres.head
                medicoRef ! Medico.Atender(pacienteNombre, pacienteRef)
                pacienteRef ! Paciente.ConsultaAsignada(medicoNombre, medicoRef)
                context.become(gestionar(libres - medicoNombre, ocupados + (medicoNombre -> medicoRef)))
            }
        case MedicoLibre(nombre) => 
            val ref = ocupados(nombre) //lo busco en ocupados
            context.become(gestionar(libres + (nombre -> ref), ocupados - nombre))
    }
    
}
