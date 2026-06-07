object Auction{
    case class ReceiveBid(id: String, amount: Double)
    case object TrackHighestOne //simpemente busca por adentro el más alto y devuelve al organizer
    case class StartAuction(organizer: ActorRef)
    case object Close //se cierra a sí misma
    case object Winner
}

//no necesita objeto propio, para enviar usa el receive
//case class SendBids(id: String, amount: Double)
object Organizer{
    case class ReceiveResult(id: String, amount: Double)
}


class Auction extends actor{ //📍
    
    import Auction._
    
    def receive: Receive = open(Map.empty) //closed(id)
    
    def open(bidders: Map[String, Double]): Receive = {
        
        case ReceiveBid(id, amount) => context.become(open(bidders + (id -> amount)))
        
        case TrackHighestOne => bidders.maxBy(_._2)._1
        
        case Close => 
            val winner = bidders.maxBy(_._2) //no me conviene mensaje amí mismo
            context.become(closed(winner)) //📍 lo calculé arriba
    }
    
    def closed(winner: (String, Double)): Receive = {
        
        case Winner => sender() ! Organizer.ReceiveResult(winner._1, winner._2)
        
        case StartAuction(duration: FiniteDuration) => 
        context.system.scheduler.scheduleOnce(duration, self, Close) //📍
        context.become(open(Map.empty))
    }
}


class Bidder(auction: ActorRef, id: String, maxAmount: Int) extends Actor {
    
    private val random = new Random()
    
    def receive: Receive = {
        case SendBid =>
            auction ! ReceiveBid(id, random.nextInt(maxAmount).toDouble)
    }
}

class Organizer extends Actor{
    
    import Organizer._
    
    //va entro de receive pq existe un único estado
    def receive: Receive = {
        case ReceiveResult(id, amount) => println(s"$id ganó con una oferta de $amount")
    }
    
}
