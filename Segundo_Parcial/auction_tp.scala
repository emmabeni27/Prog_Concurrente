import akka.actor.{Actor, ActorRef, ActorSystem, Props, SupervisorStrategy, OneForOneStrategy, Terminated}
import akka.actor.SupervisorStrategy._
import scala.concurrent.duration._
import scala.util.Random

// --- Mensajes ---
object Messages {
  // Seller -> AuctionHouse
  case class Register(item: AuctionItem, seller: ActorRef)

  // AuctionHouse -> Bidders
  case class AuctionStarted(auctionRef: ActorRef, item: AuctionItem)

  // Bidder -> Auction
  case class Bid(bidder: ActorRef, amount: Double)

  // Auction -> Bidder
  case class Outbid(newHighest: Double)
  case class YouWon(item: AuctionItem, amount: Double)

  // Auction -> Seller
  case class ItemSold(item: AuctionItem, amount: Double)
  case class ItemNotSold(item: AuctionItem)

  // Auction -> PaymentProcessor
  case class ProcessPayment(winner: ActorRef, amount: Double, auctionId: String)

  // PaymentProcessor -> AuctionHouse
  case class PaymentConfirmed(auctionId: String, winner: ActorRef, amount: Double)
  case class PaymentFailed(auctionId: String, winner: ActorRef, amount: Double)

  // AuctionHouse -> Seller
  case class PaymentSuccess(auctionId: String)
  case class PaymentFailure(auctionId: String)

  // interno Auction
  case object Close
}

// --- Item ---
case class AuctionItem(name: String, basePrice: Double, duration: Int)

// --- Seller ---
class Seller(auctionHouse: ActorRef) extends Actor {
  import Messages._

  def receive: Receive = {
    case Register(item, _) =>
      println(s"[${self.path.name}] Registering item: ${item.name}")
      auctionHouse ! Register(item, self)

    case ItemSold(item, amount) =>
      println(s"[${self.path.name}] Item '${item.name}' sold for $$$amount!")

    case ItemNotSold(item) =>
      println(s"[${self.path.name}] Item '${item.name}' not sold.")

    case PaymentSuccess(auctionId) =>
      println(s"[${self.path.name}] Payment confirmed for auction $auctionId")

    case PaymentFailure(auctionId) =>
      println(s"[${self.path.name}] Payment failed for auction $auctionId")
  }
}

// --- AuctionHouse ---
class AuctionHouse(bidders: List[ActorRef]) extends Actor {
  import Messages._

  // supervisa las subastas: si crashean las reinicia
  override val supervisorStrategy: SupervisorStrategy =
    OneForOneStrategy(maxNrOfRetries = 3, withinTimeRange = 1.minute) {
      case _: Exception => Restart
    }

  def receive: Receive = state(Map.empty)

  def state(auctions: Map[String, (ActorRef, ActorRef)]): Receive = {
    // auctions: auctionId -> (auctionRef, sellerRef)

    case Register(item, seller) =>
      val auctionId = s"AUC-${System.currentTimeMillis()}"
      val auction   = context.actorOf(Props(new Auction(auctionId, item, seller, self)), auctionId)
      context.watch(auction)
      println(s"[AuctionHouse] New auction $auctionId: '${item.name}' (base: $$${item.basePrice}, ${item.duration}s)")
      println(s"[AuctionHouse] Notifying ${bidders.size} registered bidders")
      bidders.foreach(_ ! AuctionStarted(auction, item))
      context.become(state(auctions + (auctionId -> (auction, seller))))

    case PaymentConfirmed(auctionId, winner, amount) =>
      println(s"[AuctionHouse] Payment confirmed for $auctionId")
      auctions.get(auctionId).foreach { case (_, seller) =>
        seller ! PaymentSuccess(auctionId)
      }
      context.become(state(auctions - auctionId))

    case PaymentFailed(auctionId, winner, amount) =>
      println(s"[AuctionHouse] Payment failed for $auctionId — notifying seller")
      auctions.get(auctionId).foreach { case (_, seller) =>
        seller ! PaymentFailure(auctionId)
      }
      context.become(state(auctions - auctionId))

    case Terminated(auctionRef) =>
      println(s"[AuctionHouse] Auction ${auctionRef.path.name} terminated")
  }
}

// --- Auction ---
class Auction(auctionId: String, item: AuctionItem, seller: ActorRef, auctionHouse: ActorRef) extends Actor {
  import Messages._
  import context.dispatcher

  val timer = context.system.scheduler.scheduleOnce(item.duration.seconds, self, Close)

  def receive: Receive = open(item.basePrice, None, None)

  // estado: oferta actual, postor actual, segundo mejor postor
  def open(currentBid: Double, currentBidder: Option[ActorRef], secondBidder: Option[ActorRef]): Receive = {

    case Bid(bidder, amount) =>
      if (amount > currentBid) {
        println(s"[Auction-$auctionId] Bid $$$amount from ${bidder.path.name} -> Accepted")
        currentBidder.foreach { prev =>
          println(s"[Auction-$auctionId] Notifying ${prev.path.name}: outbid! (new highest: $$$amount)")
          prev ! Outbid(amount)
        }
        context.become(open(amount, Some(bidder), currentBidder))
      } else {
        println(s"[Auction-$auctionId] Bid $$$amount from ${bidder.path.name} -> Rejected (current: $$$currentBid)")
      }

    case Close =>
      currentBidder match {
        case Some(winner) =>
          println(s"[Auction-$auctionId] Time's up! Winner: ${winner.path.name} at $$$currentBid")
          seller ! ItemSold(item, currentBid)
          winner ! YouWon(item, currentBid)
          val processor = context.actorOf(Props(new PaymentProcessor(auctionHouse)), s"payment-$auctionId")
          processor ! ProcessPayment(winner, currentBid, auctionId)
          context.become(awaitingPayment)

        case None =>
          println(s"[Auction-$auctionId] Time's up! No offers.")
          seller ! ItemNotSold(item)
          context.stop(self)
      }
  }

  def awaitingPayment: Receive = {
    case _ => // ignora nuevas ofertas mientras espera el pago
  }

  override def postStop(): Unit = {
    timer.cancel()
    println(s"[Auction-$auctionId] Stopped, timer cancelled")
  }
}

// --- PaymentProcessor ---
class PaymentProcessor(auctionHouse: ActorRef) extends Actor {
  import Messages._

  // supervisa sus propios reintentos
  override val supervisorStrategy: SupervisorStrategy =
    OneForOneStrategy(maxNrOfRetries = 3, withinTimeRange = 30.seconds) {
      case _: Exception => Restart
    }

  def receive: Receive = {
    case ProcessPayment(winner, amount, auctionId) =>
      println(s"[PaymentProcessor] Processing payment for $auctionId — $$${amount} from ${winner.path.name}")
      val success = Random.nextBoolean()
      if (success) {
        println(s"[PaymentProcessor] Payment confirmed for $auctionId")
        auctionHouse ! PaymentConfirmed(auctionId, winner, amount)
      } else {
        println(s"[PaymentProcessor] Payment failed for $auctionId")
        auctionHouse ! PaymentFailed(auctionId, winner, amount)
      }
      context.stop(self)
  }
}

// --- Bidder ---
class Bidder extends Actor {
  import Messages._

  def receive: Receive = state(500.0 + Random.nextDouble() * 500.0)

  def state(balance: Double): Receive = {
    case AuctionStarted(auction, item) =>
      println(s"[${self.path.name}] Notified of auction: '${item.name}'")
      val bid = item.basePrice + Random.nextDouble() * 200.0
      println(s"[${self.path.name}] Placing bid $$${"%.2f".format(bid)} on '${item.name}'")
      auction ! Bid(self, bid)

    case Outbid(newHighest) =>
      println(s"[${self.path.name}] Outbid! New highest: $$$newHighest")

    case YouWon(item, amount) =>
      println(s"[${self.path.name}] Won '${item.name}' for $$$amount!")
  }
}

// --- Main ---
object Main extends App {
  import Messages._

  val system = ActorSystem("AuctionSystem")

  val bidder1 = system.actorOf(Props[Bidder], "Bidder-1")
  val bidder2 = system.actorOf(Props[Bidder], "Bidder-2")
  val bidder3 = system.actorOf(Props[Bidder], "Bidder-3")

  val auctionHouse = system.actorOf(
    Props(new AuctionHouse(List(bidder1, bidder2, bidder3))),
    "AuctionHouse"
  )

  val seller1 = system.actorOf(Props(new Seller(auctionHouse)), "Seller-1")
  val seller2 = system.actorOf(Props(new Seller(auctionHouse)), "Seller-2")

  // múltiples subastas concurrentes
  seller1 ! Register(AuctionItem("Laptop", 100.0, 10), seller1)
  seller2 ! Register(AuctionItem("Headphones", 50.0, 8), seller2)
  seller1 ! Register(AuctionItem("Monitor", 200.0, 12), seller1)

  // espera que terminen y cierra
  Thread.sleep(20000)
  system.terminate()
  system.whenTerminated.foreach(_ => println("System shut down"))(system.dispatcher)
}
