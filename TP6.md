Seller — registra items, recibe resultado de la subasta y confirmación de pago
AuctionHouse — crea subastas, notifica postores, maneja resultados de pagos
Auction — maneja ofertas, se cierra con el timer, crea el PaymentProcessor
PaymentProcessor — hijo de Auction, intenta cobrar y le avisa a AuctionHouse
Bidder — recibe notificaciones, hace ofertas, recibe resultado si ganó 

AuctionHouse
    └── Auction (hijo, watched con context.watch)
            └── PaymentProcessor (hijo)
Recuperación ante fallas:

Auction crashea → AuctionHouse lo reinicia con OneForOneStrategy(Restart) — pero acá hay un problema real: al reiniciarse pierde el estado (oferta actual, postor actual). Por eso la pregunta de reflexión de la consigna pregunta exactamente eso. La respuesta honesta es que Restart no tiene mucho sentido para Auction — sería mejor Stop y notificar al vendedor que hubo un error.
PaymentProcessor crashea → Auction lo reinicia hasta 3 veces. Si sigue fallando, escala a AuctionHouse.
AuctionHouse detecta que Auction terminó → via Terminated del context.watch, limpia el mapa de subastas activas.

El log sale de los prints del programa

1. Restart y estado de Auction
Al reiniciarse, el actor pierde todo su estado — la oferta actual y el historial de postores se borran. No tiene sentido usar Restart para Auction porque reiniciar una subasta desde cero en medio del proceso sería incorrecto: los postores que ya ofertaron no saben qué pasó. La alternativa más razonable es usar Stop y notificar al vendedor que la subasta falló, o persistir el estado fuera del actor (por ejemplo con Akka Persistence) para poder recuperarlo tras el reinicio.
2. Notificación "te superaron" perdida
Si la notificación se pierde, el postor queda con una visión incorrecta del estado de la subasta. Una solución a nivel de diseño sería que cada Bid incluya un ID único y que Auction espere un Ack del postor notificado — si no llega en cierto tiempo, reintenta. Otra alternativa más simple es que los postores consulten periódicamente el estado actual de la subasta en lugar de depender de notificaciones push.
