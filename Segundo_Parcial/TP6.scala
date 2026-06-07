import akka.actor.{Actor, ActorRef, ActorSystem, Props}
import scala.concurrent.duration._

// --- Mensajes ---
object Messages {
  case class Crear(item: AuctionItem, seller: ActorRef)
  case class Bid(bidder: ActorRef, amount: Double)
  case class Avisar(msg: String)
  case class Pay(amount: Double)
  case class Won(bidder: ActorRef, amount: Double)
  case object NoOffers
  case object Close
}

// --- Item ---
case class AuctionItem(name: String, basePrice: Double, duration: Int)

// --- Seller ---
class Seller(auctionHouse: ActorRef) extends Actor {
  import Messages._

  def receive: Receive = {
    case Won(bidder, amount) => println(s"Vendido por $amount")
    case NoOffers            => println("Sin ofertas")
  }

  def register(item: AuctionItem): Unit = {
    auctionHouse ! Crear(item, self)
  }
}

// --- AuctionHouse ---
class AuctionHouse(bidders: List[ActorRef]) extends Actor {
  import Messages._

  def receive: Receive = state(Map.empty)

  def state(auctions: Map[AuctionItem, ActorRef]): Receive = {
    case Crear(item, seller) =>
      val auction = context.actorOf(Props(new Auction(item, seller)))
      context.become(state(auctions + (item -> auction)))
      self ! Avisar(s"Nueva subasta: ${item.name}")

    case Avisar(msg) =>
      bidders.foreach(_ ! Avisar(msg))
  }
}

// --- Auction ---
class Auction(item: AuctionItem, seller: ActorRef) extends Actor {
  import Messages._
  import context.dispatcher

  context.system.scheduler.scheduleOnce(item.duration.seconds, self, Close)

  def receive: Receive = state(item.basePrice, None)

  def state(currentBid: Double, currentBidder: Option[ActorRef]): Receive = {
    case Bid(bidder, amount) =>
      if (amount > currentBid) {
        currentBidder.foreach(_ ! "Te superaron")
        context.become(state(amount, Some(bidder)))
      }

    case Close =>
      currentBidder match {
        case Some(bidder) =>
          seller ! Won(bidder, currentBid)
          bidder ! Pay(currentBid)
        case None =>
          seller ! NoOffers
      }
      context.stop(self)
  }
}

// --- Bidder ---
class Bidder(balance: Double) extends Actor {
  import Messages._

  def receive: Receive = state(balance)

  def state(currentBalance: Double): Receive = {
    case Avisar(msg) => println(s"Bidder notificado: $msg")

    case Pay(amount) =>
      if (amount <= currentBalance) {
        println(s"Pago exitoso de $amount")
        context.become(state(currentBalance - amount))
      } else {
        println("Fondos insuficientes")
      }
  }

  def enviar(auction: ActorRef, amount: Double): Unit = {
    auction ! Bid(self, amount)
  }
}

// --- Main ---
object Main extends App {
  import Messages._

  val system = ActorSystem("AuctionSystem")

  val bidder1 = system.actorOf(Props(new Bidder(500.0)), "bidder1")
  val bidder2 = system.actorOf(Props(new Bidder(1000.0)), "bidder2")

  val auctionHouse = system.actorOf(Props(new AuctionHouse(List(bidder1, bidder2))), "auctionHouse")
  val seller       = system.actorOf(Props(new Seller(auctionHouse)), "seller")

  val item = AuctionItem("Cuadro", 100.0, 10)
  auctionHouse ! Crear(item, seller)
}
